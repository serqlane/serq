use super::Expression;

/// An expression that involves binary or logical operators.
#[derive(Clone, Debug)]
pub enum Operator {
    ArithmeticLogical(ArithmeticLogical),
    Comparison(Comparison),
    CompoundAssignment(CompoundAssignment),
    Boolean(Boolean),
    Negation(Negation),
    Assignment(Assignment),
    AddressOf(AddressOf),
    Dereference(Dereference),
}

/// An expression that combines two expresssions with an operator.
///
/// E.g. `a + b`, `1 << 3`.
#[derive(Clone, Debug)]
pub struct ArithmeticLogical {
    pub lhs: Box<Expression>,
    pub op: ArithmeticLogicalOperator,
    pub rhs: Box<Expression>,
}

/// An operator in an [`ArithmeticLogical`] expression.
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

/// An expression that compares two other expressions.
///
/// E.g. `a <= b`, `(5 + 1) < (5 * 2)`.
#[derive(Clone, Debug)]
pub struct Comparison {
    pub lhs: Box<Expression>,
    pub op: ComparisonOperator,
    pub rhs: Box<Expression>,
}

/// An operator in a [`Comparison`] expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComparisonOperator {
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

/// An expression that combines arithmetic and logical operators on
/// the values of `lhs` and `rhs` with an assignment to `lhs`.
///
/// E.g. `a += 5`.
#[derive(Clone, Debug)]
pub struct CompoundAssignment {
    pub lhs: Box<Expression>,
    pub op: CompoundAssignmentOperator,
    pub rhs: Box<Expression>,
}

/// An operator in a [`CompoundAssignment`] expression.
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

/// An expression that combines two boolean expressions `lhs` and
/// `rhs` with a logical operator.
///
/// E.g. `a && b`.
#[derive(Clone, Debug)]
pub struct Boolean {
    pub lhs: Box<Expression>,
    pub op: BooleanOperator,
    pub rhs: Box<Expression>,
}

/// An operator in a [`Boolean`] expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanOperator {
    And,
    Or,
}

/// An expression that negates another expression.
///
/// E.g. `-(5 + 2)`, `!(a && b && c)`.
#[derive(Clone, Debug)]
pub struct Negation {
    pub op: NegationOperator,
    pub expr: Box<Expression>,
}

/// An operator in a [`Negation`] expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NegationOperator {
    Negation,
    LogicalNot,
    BitwiseNot,
}

/// Assigns the value of `rhs` to `lhs`.
///
/// E.g. `a = 5 + 2`.
#[derive(Clone, Debug)]
pub struct Assignment {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

/// An expression that creates a reference to the evaluation of another
/// expression.
///
/// E.g. `&a`.
#[derive(Clone, Debug)]
pub struct AddressOf {
    pub expr: Box<Expression>,
}

/// An expression that dereferences another expression.
///
/// E.g. `*ptr`.
#[derive(Clone, Debug)]
pub struct Dereference {
    pub expr: Box<Expression>,
}
