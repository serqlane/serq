use std::iter::Peekable;

use crate::{
    diagnostic::SourceSpan,
    lexer::{Lexer, Token, TokenKind},
};

mod expr;

/// Parser for a Serqlane source file.
///
/// Drives a [`Lexer`] to obtain tokens, and parses them into the
/// Abstract Syntax Tree defined in [`crate::ast`].
#[derive(Clone, Debug)]
pub struct Parser<'src> {
    source: &'src str,
    lexer: Peekable<Lexer<'src>>,
}

impl<'src> Parser<'src> {
    /// Creates a new parser for a given piece of source code.
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            lexer: Lexer::new(source).peekable(),
        }
    }

    fn text(&self, span: SourceSpan) -> &'src str {
        &self.source[span]
    }

    fn peek(&mut self) -> TokenKind {
        self.lexer
            .peek()
            .map(|t| t.kind())
            .unwrap_or(TokenKind::Eof)
    }

    fn at(&mut self, token: TokenKind) -> bool {
        self.peek() == token
    }

    fn next(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    // TODO: Proper error handling.
    fn expect(&mut self, token: TokenKind) {
        let actual = self.peek();
        if actual == token {
            self.next();
        } else {
            panic!("unexpected token: {actual:?}")
        }
    }
}
