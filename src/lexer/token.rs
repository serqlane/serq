#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Token {
    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftBracket,
    /// ]
    RightBracket,

    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// %
    Percent,
    /// &
    And,
    /// |
    Or,
    /// ^
    Caret,
    /// <<
    Shl,
    /// >>
    Shr,
    /// +=
    PlusEq,
    /// -=
    MinusEq,
    /// *=
    StarEq,
    /// /=
    SlashEq,
    /// %=
    PercentEq,
    /// &=
    AndEq,
    /// |=
    OrEq,
    /// ^=
    CaretEq,
    /// <<=
    ShlEq,
    /// >>=
    ShrEq,
    /// &&
    AndAnd,
    /// ||
    OrOr,
    /// ++
    PlusPlus,
    /// --
    MinusMinus,
    /// <
    Lt,
    /// >
    Gt,
    /// =
    Eq,
    /// !
    Bang,
    /// ==
    EqEq,
    /// !=
    BangEq,
    /// <=
    LtEq,
    /// >=
    GtEq,
    /// ~
    Tilde,
    /// .
    Dot,
    /// :
    Colon,
    /// ,
    Comma,
    /// ;
    Semicolon,

    /// An identifier.
    Identifier,
    /// A string literal.
    String,
    /// A number literal.
    Number,

    /// break
    Break,
    /// const
    Const,
    /// continue
    Continue,
    /// else
    Else,
    /// enum
    Enum,
    /// false
    False,
    /// for
    For,
    /// fn
    Fn,
    /// if
    If,
    /// let
    Let,
    /// mut
    Mut,
    /// pub
    Pub,
    /// return
    Return,
    /// true
    True,
    /// while
    While,

    /// An error occurred during lexing.
    Error,
    /// End of input was reached.
    Eof,
}
