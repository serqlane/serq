//! Defines structures for describing statements.

use super::{Ident, Item, expr::Expression};

#[derive(Clone, Debug)]
pub enum Statement {
    Item(Item),
    Variable {
        ident: Ident,
        expr: Expression,
        mutable: bool,
    },
    Expression(Expression),
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: Ident,
    pub args: Box<[FunctionArg]>,
    pub ret: Option<Ident>,
    pub block: Box<[Statement]>,
}

#[derive(Clone, Debug)]
pub struct FunctionArg {
    pub name: Ident,
    pub typ: Ident,
}
