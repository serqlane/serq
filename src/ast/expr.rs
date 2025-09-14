//! Defines structures for describing expressions.

use crate::lexer::TokenKind;

use super::{Ident, stmt::Statement};

#[derive(Clone, Debug)]
pub enum Expression {
    Ident(Ident),

    Block {
        stmts: Box<[Statement]>,
    },

    Index {
        cont: Box<Expression>,
        idx: Box<Expression>,
    },

    Call {
        func: Box<Expression>,
        params: Box<[Expression]>,
    },

    Literal(Literal),

    Operator(OperatorExpression),
}

#[derive(Clone, Debug)]
pub enum Literal {
    Int(u64),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub enum OperatorExpression {
    // `a + b`, `1 << 3`
    ArithmeticLogical {
        lhs: Box<Expression>,
        op: ArithmeticLogicalOperator,
        rhs: Box<Expression>,
    },

    // `a <= b`, `(5 + 1) < (5 * 2)`
    Comparison {
        lhs: Box<Expression>,
        op: ComparisonOperator,
        rhs: Box<Expression>,
    },

    // `a += 5`
    CompoundAssignment {
        lhs: Box<Expression>,
        op: CompoundAssignmentOperator,
        rhs: Box<Expression>,
    },

    // `a && b`
    Boolean {
        lhs: Box<Expression>,
        op: BooleanOperator,
        rhs: Box<Expression>,
    },

    // `-(5 + 2)`, `!(a && b)`
    Negation {
        op: NegationOperator,
        expr: Box<Expression>,
    },

    // `a = 5 + 2`
    Assignment {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    // `&a`
    AddressOf {
        expr: Box<Expression>,
    },

    // `*ptr`
    Dereference {
        expr: Box<Expression>,
    },
}

impl OperatorExpression {
    pub fn prefix(op: TokenKind, expr: Expression) -> Self {
        let expr = Box::new(expr);
        match op {
            TokenKind::Minus | TokenKind::Bang | TokenKind::Tilde => Self::Negation {
                op: NegationOperator::from(op),
                expr,
            },
            TokenKind::And => Self::AddressOf { expr },
            TokenKind::Star => Self::Dereference { expr },
            _ => unreachable!(),
        }
    }

    pub fn infix(lhs: Expression, op: TokenKind, rhs: Expression) -> Self {
        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);
        match op {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Caret
            | TokenKind::Shl
            | TokenKind::Shr => Self::ArithmeticLogical {
                lhs,
                op: ArithmeticLogicalOperator::from(op),
                rhs,
            },
            TokenKind::EqEq
            | TokenKind::BangEq
            | TokenKind::Lt
            | TokenKind::LtEq
            | TokenKind::Gt
            | TokenKind::GtEq => Self::Comparison {
                lhs,
                op: ComparisonOperator::from(op),
                rhs,
            },
            TokenKind::Eq => Self::Assignment { lhs, rhs },
            TokenKind::AndAnd | TokenKind::OrOr => Self::Boolean {
                lhs,
                op: BooleanOperator::from(op),
                rhs,
            },
            TokenKind::PlusEq
            | TokenKind::MinusEq
            | TokenKind::StarEq
            | TokenKind::SlashEq
            | TokenKind::PercentEq
            | TokenKind::ShlEq
            | TokenKind::ShrEq
            | TokenKind::AndEq
            | TokenKind::OrEq
            | TokenKind::CaretEq => Self::CompoundAssignment {
                lhs,
                op: CompoundAssignmentOperator::from(op),
                rhs,
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArithmeticLogicalOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

impl From<TokenKind> for ArithmeticLogicalOperator {
    fn from(op: TokenKind) -> Self {
        match op {
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Multiply,
            TokenKind::Slash => Self::Divide,
            TokenKind::Percent => Self::Modulo,
            TokenKind::And => Self::And,
            TokenKind::Or => Self::Or,
            TokenKind::Caret => Self::Xor,
            TokenKind::Shl => Self::Shl,
            TokenKind::Shr => Self::Shr,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComparisonOperator {
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

impl From<TokenKind> for ComparisonOperator {
    fn from(op: TokenKind) -> Self {
        match op {
            TokenKind::EqEq => Self::Eq,
            TokenKind::BangEq => Self::NotEq,
            TokenKind::Lt => Self::Lt,
            TokenKind::LtEq => Self::LtEq,
            TokenKind::Gt => Self::Gt,
            TokenKind::GtEq => Self::GtEq,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompoundAssignmentOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

impl From<TokenKind> for CompoundAssignmentOperator {
    fn from(op: TokenKind) -> Self {
        match op {
            TokenKind::PlusEq => Self::Plus,
            TokenKind::MinusEq => Self::Minus,
            TokenKind::StarEq => Self::Multiply,
            TokenKind::SlashEq => Self::Divide,
            TokenKind::PercentEq => Self::Modulo,
            TokenKind::ShlEq => Self::Shl,
            TokenKind::ShrEq => Self::Shr,
            TokenKind::AndEq => Self::And,
            TokenKind::OrEq => Self::Or,
            TokenKind::CaretEq => Self::Xor,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanOperator {
    And,
    Or,
}

impl From<TokenKind> for BooleanOperator {
    fn from(op: TokenKind) -> Self {
        match op {
            TokenKind::AndAnd => Self::And,
            TokenKind::OrOr => Self::Or,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NegationOperator {
    Negation,
    LogicalNot,
    BitwiseNot,
}

impl From<TokenKind> for NegationOperator {
    fn from(op: TokenKind) -> Self {
        match op {
            TokenKind::Minus => Self::Negation,
            TokenKind::Bang => Self::LogicalNot,
            TokenKind::Tilde => Self::BitwiseNot,
            _ => unreachable!(),
        }
    }
}
