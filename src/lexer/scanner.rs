use std::str::CharIndices;

use super::{
    SourceSpan, Token,
    keywords::{MAX_KEYWORD_LEN, is_keyword},
};

#[derive(Clone, Debug)]
pub struct Scanner<'src> {
    // Iterator over a piece of source code that provides byte
    // offsets for each consumed character.
    source: CharIndices<'src>,

    // The last token that was read from the source.
    // This is used for error tracking and to determine when
    // an implicit semicolon should be injected into the stream.
    previous: Token,
}

fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    // The set is stable so it is ok to hardcode the values.
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

fn should_terminate(token: Token) -> bool {
    matches!(
        token,
        // Punctuation
        Token::RightParen
        | Token::RightBrace
        | Token::RightBracket

        // Identifiers and literals
        | Token::Identifier
        | Token::String
        | Token::Number

        // Keywords
        | Token::Break
        | Token::Continue
        | Token::Return

        // Operators
        | Token::PlusPlus
        | Token::MinusMinus
    )
}

fn is_ident1(c: char) -> bool {
    c == '_' || unicode_ident::is_xid_start(c)
}

fn is_ident2(c: char) -> bool {
    unicode_ident::is_xid_continue(c)
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source: source.char_indices(),
            previous: Token::Eof,
        }
    }

    #[inline]
    fn offset(&self) -> usize {
        self.source.offset()
    }

    #[inline]
    fn peek(&self) -> Option<char> {
        // NOTE: Cloning the iterator is very cheap and has less
        // overhead than going through the Peekable trait.
        let mut it = self.source.clone();
        it.next().map(|v| v.1)
    }

    #[inline]
    fn consume(&mut self) -> Option<char> {
        self.source.next().map(|v| v.1)
    }

    #[inline]
    fn match1(&mut self, c: char, a: Token, b: Token) -> Token {
        if self.peek() == Some(c) {
            self.consume();
            a
        } else {
            b
        }
    }

    #[inline]
    fn match2(&mut self, c1: char, a: Token, c2: char, b: Token, c: Token) -> Token {
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

    fn whitespace(&mut self) -> bool {
        let mut inject_semicolon = false;

        // TODO: Remove comments here too.
        while let Some(c) = self.peek() {
            match c {
                c if is_whitespace(c) => {
                    self.consume();

                    // Check if we need to insert an automatic semicolon.
                    if c == '\n' && should_terminate(self.previous) {
                        inject_semicolon = true;
                    }
                }

                _ => break,
            }
        }

        inject_semicolon
    }

    fn string(&mut self) -> Token {
        // Consume characters of the string literal.
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            self.consume();
        }

        // Consume the closing quote at the end.
        if self.consume().is_some() {
            Token::String
        } else {
            Token::Error
        }
    }

    fn number(&mut self) -> Token {
        // TODO: Handle more complex number representations.
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            self.consume();
        }

        Token::Number
    }

    fn name(&mut self, first: char) -> Token {
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

        // Try to match a potential keyword using a perfect hash function.
        if keyword_candidate && cursor > 0 {
            if let Some(token) = is_keyword(keyword_buf, cursor) {
                return token;
            }
        }

        Token::Identifier
    }

    fn scan(&mut self) -> (Token, SourceSpan) {
        // First, consume whitspace and inject a semicolon, if necessary.
        // Note that a call consumes all available whitespace so the next
        // call is guaranteed to return false. This is important to make
        // sure we do not get stuck in an infinite semicolon loop.
        if self.whitespace() {
            let off = self.offset();
            return (Token::Semicolon, SourceSpan::new(off - 1, off));
        }

        let start = self.offset();

        // Consume the next non-whitespace character or report EOF if no
        // more input is available to us.
        let Some(c) = self.consume() else {
            return (Token::Eof, SourceSpan::new(start, start));
        };

        let token = match c {
            c if is_ident1(c) => self.name(c),
            c if c.is_ascii_digit() => self.number(),

            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '+' => self.match2('=', Token::PlusEq, '+', Token::PlusPlus, Token::Plus),
            '-' => self.match2('=', Token::MinusEq, '-', Token::MinusMinus, Token::Minus),
            '*' => self.match1('=', Token::StarEq, Token::Star),
            '/' => self.match1('=', Token::SlashEq, Token::Slash),
            '%' => self.match1('=', Token::PercentEq, Token::Percent),
            '&' => self.match2('=', Token::AndEq, '&', Token::AndAnd, Token::And),
            '|' => self.match2('=', Token::OrEq, '|', Token::OrOr, Token::Or),
            '^' => self.match1('=', Token::CaretEq, Token::Caret),
            '<' => {
                if self.peek() == Some('<') {
                    self.consume();
                    self.match1('=', Token::ShlEq, Token::Shl)
                } else {
                    self.match1('=', Token::LtEq, Token::Lt)
                }
            }
            '>' => {
                if self.peek() == Some('>') {
                    self.consume();
                    self.match1('=', Token::ShrEq, Token::Shr)
                } else {
                    self.match1('=', Token::GtEq, Token::Gt)
                }
            }
            '=' => self.match1('=', Token::EqEq, Token::Eq),
            '!' => self.match1('=', Token::BangEq, Token::Bang),
            '~' => Token::Tilde,
            '.' => Token::Dot,
            ':' => Token::Colon,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '\"' => self.string(),

            _ => Token::Error,
        };
        self.previous = token;

        let end = self.offset();
        (token, SourceSpan::new(start, end))
    }
}

impl<'src> Iterator for Scanner<'src> {
    type Item = (Token, SourceSpan);

    fn next(&mut self) -> Option<Self::Item> {
        // If an error is already set, we stop iteration there
        // to avoid producing a cascade of more errors.
        if self.previous == Token::Error {
            return None;
        }

        let (token, span) = self.scan();
        if token == Token::Eof {
            // Convert EOF to an error to stop iteration.
            self.previous = Token::Error;
        }

        Some((token, span))
    }
}
