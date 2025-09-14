//! Defines the Abstract Syntax Tree (AST) to hold a source program.
//!
//! The AST is obtained through the parsing stage implemented in the
//! [`crate::parser`] module and represents a machine-readable
//! version of a program for use in subsequent compiler stages.
//!
//! The structures are intentionally kept very simple to support
//! ergonomics and convenience when manipulating the AST. We will
//! reuse that same AST in the subsequent passes by filling optional
//! fields as we go (e.g. with type information from type inference).

use crate::{
    diagnostic::SourceSpan,
    lexer::{Token, TokenKind},
};

pub mod expr;

pub mod stmt;

#[derive(Clone, Debug)]
pub struct Ident {
    pub span: SourceSpan,
}

impl From<Token> for Ident {
    fn from(t: Token) -> Self {
        debug_assert_eq!(t.kind(), TokenKind::Identifier);
        Self { span: t.span() }
    }
}

#[derive(Clone, Debug)]
pub enum Item {
    Function(stmt::Function),
}
