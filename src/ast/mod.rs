//! Defines the Abstract Syntax Tree (AST) to hold a source program.
//!
//! The AST is obtained through the parsing stage implemented in the
//! [`crate::parser`] module and will be used in subsequent stages.
//!
//! The structures are intentionally kept very simple to support
//! ergonomics and convenience when working with the AST. We will
//! reuse that same AST in the subsequent passes by filling optional
//! fields as we go (e.g. with type information from type inference).

pub mod expr;
