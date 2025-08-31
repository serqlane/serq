use super::Expression;

/// An expression that indexes into the `lhs` array with an expression.
///
/// E.g. `container[x + 3]`.
#[derive(Clone, Debug)]
pub struct Index {
    pub base: Box<Expression>,
    pub index: Box<Expression>,
}
