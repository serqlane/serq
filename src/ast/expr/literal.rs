/// A literal in the source code.
///
/// E.g. `5`, `true`, `"foo"`.
#[derive(Clone, Debug)]
pub enum Literal {
    Integer(u64),
    Bool(bool),
}
