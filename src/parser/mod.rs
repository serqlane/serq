//! Implements parsing of a Serqlane program to an AST.
//!
//! Given a piece of source code, the parser will tokenize it using
//! the [`crate::lexer`] module and validate the syntax of the program.
//! On success, a list of [`Item`]s encoding the program's structure
//! is returned.
//!
//! Implementation-wise, this is a hand-rolled recursive descent parser
//! using a Pratt parsing scheme to handle expressions and precedence.
//! The resulting tree is then subject to semantic analysis.

use std::iter::Peekable;

use crate::{
    ast::{Ident, Item},
    diagnostic::SourceSpan,
    lexer::{Lexer, Token, TokenKind},
};

mod expr;
mod stmt;

#[derive(Clone, Debug)]
pub struct Parser<'src> {
    source: &'src str,
    lexer: Peekable<Lexer<'src>>,
}

impl<'src> Parser<'src> {
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

    fn eof(&mut self) -> bool {
        self.at(TokenKind::Eof)
    }

    fn next(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    fn eat(&mut self, token: TokenKind) {
        let actual = self.peek();
        if actual == token {
            self.next();
        } else {
            panic!("unexpected token: {actual:?}");
        }
    }

    fn ident(&mut self) -> Ident {
        match self.next() {
            Some(t) if t.kind() == TokenKind::Identifier => Ident::from(t),
            t => panic!("expected identifier, got: {t:?}"),
        }
    }

    pub(super) fn item(&mut self) -> Option<Item> {
        if self.at(TokenKind::Fn) {
            Some(Item::Function(self.function()))
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Vec<Item> {
        let mut items = Vec::new();
        while !self.eof() {
            items.push(self.item().expect("expected item"));
            self.eat(TokenKind::Semicolon);
        }
        items
    }
}
