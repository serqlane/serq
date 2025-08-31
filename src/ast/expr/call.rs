use super::Expression;

/// A call to a function with parameters.
///
/// E.g. `fibonacci(6)`.
#[derive(Clone, Debug)]
pub struct Call {
    pub func: Box<Expression>,
    pub params: Vec<Expression>,
}
