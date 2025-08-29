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

/// Breaks down a given piece of source code into tokens.
///
/// The only public-facing interface to drive the lexer is [`Iterator`];
/// iteration stops after [`Token::Eof`] has been yielded once.
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
    unicode_ident::is_xid_continue(c)
}

impl<'src> Lexer<'src> {
    /// Creates a new [`Lexer`] over a given string of source code.
    pub fn new(source: &'src str) -> Self {
        // Invariant: Source files need to be smaller than 4GiB so that
        // our spans can cover the entire text with u32 offsets.
        debug_assert!(u32::try_from(source.len()).is_ok());
        debug_assert!(source.len() <= u32::MAX as usize);

        Self {
            source: source.char_indices(),
            previous: TokenKind::Error,
        }
    }

    fn offset(&self) -> u32 {
        self.source.offset() as u32
    }

    fn peek(&self) -> Option<char> {
        let mut it = self.source.clone();
        it.next().map(|v| v.1)
    }

    fn peek2(&self) -> Option<char> {
        let mut it = self.source.clone();
        it.next()?;
        it.next().map(|v| v.1)
    }

    fn consume(&mut self) -> Option<char> {
        self.source.next().map(|v| v.1)
    }

    fn try_eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.consume();
            true
        } else {
            false
        }
    }

    fn match1(&mut self, c: char, a: TokenKind, b: TokenKind) -> TokenKind {
        if self.peek() == Some(c) {
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
            Some(c) if c == c1 => {
                self.consume();
                a
            }
            Some(c) if c == c2 => {
                self.consume();
                b
            }
            _ => c,
        }
    }

    fn line_comment(&mut self) -> Token {
        let start = self.offset();

        // Discard the entire line as a comment.
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.consume();
        }

        let end = self.offset();
        Token {
            kind: TokenKind::Comment,
            span: SourceSpan::from(start..end),
        }
    }

    fn multi_line_comment(&mut self) -> Token {
        let start = self.offset();

        // Consume the `/*` characters at the start.
        self.consume();
        self.consume();

        // Eat characters until we find the closing `*/`.
        while let Some(c) = self.peek() {
            if c == '*' && self.peek2() == Some('/') {
                break;
            }
            self.consume();
        }

        // Now try to consume these remaining characters.
        let star = self.consume();
        let slash = self.consume();

        let end = self.offset();
        let span = SourceSpan::from(start..end);

        let kind = match star.and(slash) {
            Some(..) => TokenKind::Comment,
            None => TokenKind::Error,
        };

        Token { kind, span }
    }

    fn whitespace(&mut self) -> Option<Token> {
        let mut token = None;
        while let Some(c) = self.peek() {
            match c {
                // When handling newlines, we need to check the last token
                // of the line to see if an implicit semicolon should be
                // injected to terminate an expression.
                '\n' => {
                    if should_terminate_expr(self.previous) {
                        let pos = self.offset();
                        token = Some(Token {
                            kind: TokenKind::Semicolon,
                            span: SourceSpan::from(pos..pos),
                        });
                    }
                    self.consume();
                }

                // Other whitespace can be trivially ignored.
                c if c.is_whitespace() => {
                    self.consume();
                }

                // Consume the contents of a line comment.
                '/' if self.peek2() == Some('/') => {
                    token = Some(self.line_comment());
                    break;
                }

                // Consume the contents of a multiline comment.
                '/' if self.peek2() == Some('*') => {
                    token = Some(self.multi_line_comment());
                    break;
                }

                _ => break,
            }
        }

        token
    }

    fn string(&mut self) -> TokenKind {
        // Consume characters of the string literal.
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            self.consume();
        }

        // Try to consume the closing quote.
        if self.consume().is_some() {
            TokenKind::String
        } else {
            TokenKind::Error
        }
    }

    fn number(&mut self) -> TokenKind {
        // TODO: Handle more complex number representations.
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }
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
        while let Some(c) = self.peek() {
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
        let Some(c) = self.consume() else {
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
                if self.try_eat('<') {
                    self.match1('=', ShlEq, Shl)
                } else {
                    self.match1('=', LtEq, Lt)
                }
            }
            '>' => {
                if self.try_eat('>') {
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
        if self.previous != TokenKind::Eof {
            Some(self.scan())
        } else {
            None
        }
    }
}
