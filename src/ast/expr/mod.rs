//! Defines structures for describing expressions.

use crate::{
    diagnostic::SourceSpan,
    lexer::{Token, TokenKind},
};

mod array;
pub use array::*;

mod call;
pub use call::*;

mod literal;
pub use literal::*;

mod operator;
pub use operator::*;

/// An identifier in the source code.
///
/// Identifiers are represented through their span in the original
/// source file. Their string value must be fetched on demand.
#[derive(Clone, Debug)]
pub struct Ident {
    pub span: SourceSpan,
}

impl From<Token> for Ident {
    fn from(t: Token) -> Self {
        Self { span: t.span() }
    }
}

/// An expression in the Serqlane language.
#[derive(Clone, Debug)]
pub enum Expression {
    Ident(Ident),
    Index(Index),
    Call(Call),
    Literal(Literal),
    Operator(Operator),
}

impl Expression {
    /// Creates a new index expression.
    ///
    /// `base` is the expression that is indexed, and `index` is the
    /// expression inside the `[]`.
    pub fn index(base: Expression, index: Expression) -> Self {
        let base = Box::new(base);
        let index = Box::new(index);
        Self::Index(Index { base, index })
    }

    /// Creates a call expression from a function expression and a vector
    /// of parameter expressions it is invoked with.
    pub fn call(func: Expression, params: Vec<Expression>) -> Self {
        let func = Box::new(func);
        Self::Call(Call { func, params })
    }

    /// Creates a prefix operator expression that matches the given `op`.
    ///
    /// # Panics
    ///
    /// This method panics if `op` is not a valid prefix operator. It is
    /// expected to be called with valid arguments only.
    pub fn prefix_operator(op: TokenKind, expr: Expression) -> Self {
        let expr = Box::new(expr);
        let op = match op {
            TokenKind::Minus | TokenKind::Bang | TokenKind::Tilde => {
                let negation = Negation {
                    op: match op {
                        TokenKind::Minus => NegationOperator::Negation,
                        TokenKind::Bang => NegationOperator::LogicalNot,
                        TokenKind::Tilde => NegationOperator::BitwiseNot,
                        _ => unreachable!(),
                    },
                    expr,
                };
                Operator::Negation(negation)
            }
            TokenKind::And => Operator::AddressOf(AddressOf { expr }),
            TokenKind::Star => Operator::Dereference(Dereference { expr }),
            _ => unreachable!(),
        };
        Self::Operator(op)
    }

    pub fn infix_operator(lhs: Expression, op: TokenKind, rhs: Expression) -> Self {
        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);
        let op = match op {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Caret
            | TokenKind::Shl
            | TokenKind::Shr => Operator::ArithmeticLogical(ArithmeticLogical {
                lhs,
                op: match op {
                    TokenKind::Plus => ArithmeticLogicalOperator::Plus,
                    TokenKind::Minus => ArithmeticLogicalOperator::Minus,
                    TokenKind::Star => ArithmeticLogicalOperator::Multiply,
                    TokenKind::Slash => ArithmeticLogicalOperator::Divide,
                    TokenKind::Percent => ArithmeticLogicalOperator::Modulo,
                    TokenKind::And => ArithmeticLogicalOperator::And,
                    TokenKind::Or => ArithmeticLogicalOperator::Or,
                    TokenKind::Caret => ArithmeticLogicalOperator::Xor,
                    TokenKind::Shl => ArithmeticLogicalOperator::Shl,
                    TokenKind::Shr => ArithmeticLogicalOperator::Shr,
                    _ => unreachable!(),
                },
                rhs,
            }),

            TokenKind::EqEq
            | TokenKind::BangEq
            | TokenKind::Lt
            | TokenKind::LtEq
            | TokenKind::Gt
            | TokenKind::GtEq => Operator::Comparison(Comparison {
                lhs,
                op: match op {
                    TokenKind::EqEq => ComparisonOperator::Eq,
                    TokenKind::BangEq => ComparisonOperator::NotEq,
                    TokenKind::Lt => ComparisonOperator::Lt,
                    TokenKind::LtEq => ComparisonOperator::LtEq,
                    TokenKind::Gt => ComparisonOperator::Gt,
                    TokenKind::GtEq => ComparisonOperator::GtEq,
                    _ => unreachable!(),
                },
                rhs,
            }),

            TokenKind::Eq => Operator::Assignment(Assignment { lhs, rhs }),

            TokenKind::AndAnd | TokenKind::OrOr => Operator::Boolean(Boolean {
                lhs,
                op: match op {
                    TokenKind::AndAnd => BooleanOperator::And,
                    TokenKind::OrOr => BooleanOperator::Or,
                    _ => unreachable!(),
                },
                rhs,
            }),

            TokenKind::PlusEq
            | TokenKind::MinusEq
            | TokenKind::StarEq
            | TokenKind::SlashEq
            | TokenKind::PercentEq
            | TokenKind::ShlEq
            | TokenKind::ShrEq
            | TokenKind::AndEq
            | TokenKind::OrEq
            | TokenKind::CaretEq => Operator::CompoundAssignment(CompoundAssignment {
                lhs,
                op: match op {
                    TokenKind::PlusEq => CompoundAssignmentOperator::Plus,
                    TokenKind::MinusEq => CompoundAssignmentOperator::Minus,
                    TokenKind::StarEq => CompoundAssignmentOperator::Multiply,
                    TokenKind::SlashEq => CompoundAssignmentOperator::Divide,
                    TokenKind::PercentEq => CompoundAssignmentOperator::Modulo,
                    TokenKind::ShlEq => CompoundAssignmentOperator::Shl,
                    TokenKind::ShrEq => CompoundAssignmentOperator::Shr,
                    TokenKind::AndEq => CompoundAssignmentOperator::And,
                    TokenKind::OrEq => CompoundAssignmentOperator::Or,
                    TokenKind::CaretEq => CompoundAssignmentOperator::Xor,
                    _ => unreachable!(),
                },
                rhs,
            }),

            _ => unreachable!(),
        };
        Self::Operator(op)
    }
}
