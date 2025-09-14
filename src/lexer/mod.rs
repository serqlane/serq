//! Implements lexical analysis of Serqlane programs.
//!
//! Lexing is the first stage of the compiler and breaks down a given
//! piece of source code into a sequence of tokens that is easier for
//! us to work with in subsequent stages:
//!
//! ```no_run
//! mut x = 5
//! ```
//!
//! is turned into the following stream:
//!
//! ```no_run
//! Mut Identifier Eq Number Semicolon
//! ```
//!
//! At the core, the [`Lexer`] allows for tokens to be streamed on demand
//! through its `Iterator<Item = Token>` interface, which can then be fed
//! into the parser. It should be noted that no parsing work is performed
//! during lexing, which makes a [`Token`] rather low-level. Consider for
//! example [`TokenKind::Number`], which doesn't specify the type of int
//! of float that was encountered.
//!
//! Serqlane supports the automatic insertion of implicit semicolons to
//! terminate expressions following the token types listed in the
//! `should_terminate_expr` helper function. Other than that, this lexer
//! is fairly conventional and doesn't have outstanding intricacies.

use std::str::CharIndices;

use crate::diagnostic::SourceSpan;

mod keywords;
use keywords::{MAX_KEYWORD_LEN, check_keyword};

mod token;
pub use token::{Token, TokenKind};

const EOF_CHAR: char = '\0';

/// Breaks down a given piece of source code into tokens.
///
/// The only public-facing interface to drive the lexer is [`Iterator`];
/// iteration stops after [`TokenKind::Eof`] has been yielded once.
///
/// See the [module documentation][crate::lexer] for more details.
#[derive(Clone, Debug)]
pub struct Lexer<'src> {
    // Iterator over a piece of source code that provides byte
    // offsets for each consumed character.
    source: CharIndices<'src>,

    // The last token that was read from the source.
    // This is used to determine when an implicit semicolon
    // should be injected into the stream.
    previous: TokenKind,
}

fn should_terminate_expr(token: TokenKind) -> bool {
    matches!(
        token,
        // Punctuation
        TokenKind::RightParen
        | TokenKind::RightBrace
        | TokenKind::RightBracket

        // Identifiers and literals
        | TokenKind::Identifier
        | TokenKind::String
        | TokenKind::Number

        // Keywords
        | TokenKind::Break
        | TokenKind::Continue
        | TokenKind::Return

        // Operators
        | TokenKind::PlusPlus
        | TokenKind::MinusMinus
    )
}

fn is_ident1(c: char) -> bool {
    c == '_' || unicode_ident::is_xid_start(c)
}

fn is_ident2(c: char) -> bool {
    if c.is_ascii() {
        // Avoid the slower `is_xid_continue` path for ASCII ranges.
        matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')
    } else {
        unicode_ident::is_xid_continue(c)
    }
}

impl<'src> Lexer<'src> {
    /// Creates a new [`Lexer`] over a given string of source code.
    pub fn new(source: &'src str) -> Self {
        // Invariant: Source files need to be smaller than 4GiB so that
        // our spans can cover the entire text with u32 offsets.
        debug_assert!(u32::try_from(source.len()).is_ok());

        Self {
            source: source.char_indices(),
            previous: TokenKind::Eof,
        }
    }

    fn offset(&self) -> u32 {
        self.source.offset() as u32
    }

    fn peek(&self) -> char {
        let mut it = self.source.clone();
        it.next().map(|v| v.1).unwrap_or(EOF_CHAR)
    }

    fn peek2(&self) -> char {
        let mut it = self.source.clone();
        it.next().and(it.next()).map(|v| v.1).unwrap_or(EOF_CHAR)
    }

    fn reached_eof(&self) -> bool {
        self.peek() == EOF_CHAR
    }

    fn consume(&mut self) -> char {
        self.source.next().map(|v| v.1).unwrap_or(EOF_CHAR)
    }

    fn match1(&mut self, c: char, a: TokenKind, b: TokenKind) -> TokenKind {
        if self.peek() == c {
            self.consume();
            a
        } else {
            b
        }
    }

    fn match2(
        &mut self,
        c1: char,
        a: TokenKind,
        c2: char,
        b: TokenKind,
        c: TokenKind,
    ) -> TokenKind {
        match self.peek() {
            v if v == c1 => {
                self.consume();
                a
            }
            v if v == c2 => {
                self.consume();
                b
            }
            _ => c,
        }
    }

    fn line_comment(&mut self) {
        while self.peek() != '\n' && !self.reached_eof() {
            self.consume();
        }
    }

    fn multi_line_comment(&mut self) {
        self.consume();
        self.consume();

        while !(self.peek() == '*' && self.peek2() == '/') {
            if self.reached_eof() {
                return;
            }

            self.consume();
        }

        self.consume();
        self.consume();
    }

    fn whitespace(&mut self) -> Option<Token> {
        let mut token = None;
        loop {
            match self.peek() {
                // When handling newlines, we need to check the last token
                // of the line to see if an implicit semicolon should be
                // injected to terminate an expression. On EOF, we do the
                // same so we don't accidentally cut a semicolon off.
                '\n' | EOF_CHAR => {
                    if should_terminate_expr(self.previous) {
                        let pos = self.offset();
                        token = Some(Token {
                            kind: TokenKind::Semicolon,
                            span: SourceSpan::from(pos..pos),
                        });
                    }
                    if self.consume() == EOF_CHAR {
                        break;
                    }
                }

                // Other whitespace can be trivially ignored.
                c if c.is_whitespace() => {
                    self.consume();
                }

                // Handle comments.
                '/' => {
                    let c2 = self.peek2();
                    if c2 == '/' {
                        self.line_comment();
                    } else if c2 == '*' {
                        self.multi_line_comment();
                    }
                }

                _ => break,
            }
        }

        token
    }

    fn string(&mut self) -> TokenKind {
        while self.peek() != '"' {
            if self.reached_eof() {
                return TokenKind::Error;
            }

            self.consume();
        }

        self.consume();
        TokenKind::String
    }

    fn number(&mut self) -> TokenKind {
        // TODO: Handle more complex number representations.
        while self.peek().is_ascii_digit() && !self.reached_eof() {
            self.consume();
        }

        TokenKind::Number
    }

    fn name(&mut self, first: char) -> TokenKind {
        let mut keyword_buf = [0u8; MAX_KEYWORD_LEN];
        let mut cursor = 0;
        let mut keyword_candidate = first.is_ascii_lowercase();

        if keyword_candidate {
            keyword_buf[0] = first as u8;
            cursor = 1;
        }

        // Consume the entire valid identifier. As long as the input
        // is a candidate for a keyword, spill it to keyword_buf.
        while !self.reached_eof() {
            let c = self.peek();
            if keyword_candidate && cursor < MAX_KEYWORD_LEN && c.is_ascii_lowercase() {
                keyword_buf[cursor] = c as u8;
                cursor += 1;
            } else if is_ident2(c) {
                keyword_candidate = false;
            } else {
                break;
            }
            self.consume();
        }

        // Try to match a potential keyword.
        if cursor > 0 && keyword_candidate {
            return check_keyword(keyword_buf);
        }

        TokenKind::Identifier
    }

    fn scan(&mut self) -> Token {
        use TokenKind::*;

        // First, consume whitspace and inject a semicolon, if necessary.
        // A call consumes all available whitespace so the next call is
        // guaranteed to not return semicolon again. This is necessary to
        // ensure we don't get stuck in an infinite semicolon loop.
        if let Some(token) = self.whitespace() {
            self.previous = token.kind;
            return token;
        }

        let start = self.offset();

        // Consume the next non-whitespace character or report EOF if no
        // more input is available to us.
        let c = self.consume();
        if c == EOF_CHAR {
            self.previous = Eof;
            return Token {
                kind: Eof,
                span: SourceSpan::from(start..start),
            };
        };

        let kind = match c {
            c if is_ident1(c) => self.name(c),
            c if c.is_ascii_digit() => self.number(),

            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            '[' => LeftBracket,
            ']' => RightBracket,
            '+' => self.match2('=', PlusEq, '+', PlusPlus, Plus),
            '-' => self.match2('=', MinusEq, '-', MinusMinus, Minus),
            '*' => self.match1('=', StarEq, Star),
            '/' => self.match1('=', SlashEq, Slash),
            '%' => self.match1('=', PercentEq, Percent),
            '&' => self.match2('=', AndEq, '&', AndAnd, And),
            '|' => self.match2('=', OrEq, '|', OrOr, Or),
            '^' => self.match1('=', CaretEq, Caret),
            '<' => {
                if self.peek() == '<' {
                    self.consume();
                    self.match1('=', ShlEq, Shl)
                } else {
                    self.match1('=', LtEq, Lt)
                }
            }
            '>' => {
                if self.peek() == '>' {
                    self.consume();
                    self.match1('=', ShrEq, Shr)
                } else {
                    self.match1('=', GtEq, Gt)
                }
            }
            '=' => self.match1('=', EqEq, Eq),
            '!' => self.match1('=', BangEq, Bang),
            '~' => Tilde,
            '.' => Dot,
            ':' => Colon,
            ',' => Comma,
            ';' => Semicolon,
            '"' => self.string(),

            _ => Error,
        };
        self.previous = kind;

        let end = self.offset();
        Token {
            kind,
            span: SourceSpan::from(start..end),
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.scan();
        if token.kind() != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    }
}
