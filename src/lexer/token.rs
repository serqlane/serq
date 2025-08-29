use crate::diagnostic::SourceSpan;

/// Represents a lexeme of the Serqlane language.
///
/// Tokens consist of a [`TokenKind`] to describe their nature,
/// and a [`SourceSpan`] to resolve the original text in the
/// source code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub(super) kind: TokenKind,
    pub(super) span: SourceSpan,
}

impl Token {
    /// Gets the [`TokenKind`] for this token.
    pub fn kind(self) -> TokenKind {
        self.kind
    }

    /// Gets the [`SourceSpan`] for this token.
    pub fn span(self) -> SourceSpan {
        self.span
    }
}

/// A low-level description of the types of tokens in a program.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,

    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `&`
    And,
    /// `|`
    Or,
    /// `^`
    Caret,
    /// `<<`
    Shl,
    /// `>>`
    Shr,
    /// `+=`
    PlusEq,
    /// `-=`
    MinusEq,
    /// `*=`
    StarEq,
    /// `/=`
    SlashEq,
    /// `%=`
    PercentEq,
    /// `&=`
    AndEq,
    /// `|=`
    OrEq,
    /// `^=`
    CaretEq,
    /// `<<=`
    ShlEq,
    /// `>>=`
    ShrEq,
    /// `&&`
    AndAnd,
    /// `||`
    OrOr,
    /// `++`
    PlusPlus,
    /// `--`
    MinusMinus,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `=`
    Eq,
    /// `!`
    Bang,
    /// `==`
    EqEq,
    /// `!=`
    BangEq,
    /// `<=`
    LtEq,
    /// `>=`
    GtEq,
    /// `~`
    Tilde,
    /// `.`
    Dot,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `;`
    Semicolon,

    /// An identifier.
    Identifier,
    /// A string literal.
    String,
    /// A number literal.
    Number,
    /// A comment in the source code.
    Comment,

    /// `break`
    Break,
    /// `const`
    Const,
    /// `continue`
    Continue,
    /// `else`
    Else,
    /// `enum`
    Enum,
    /// `false`
    False,
    /// `for`
    For,
    /// `fn`
    Fn,
    /// `if`
    If,
    /// `let`
    Let,
    /// `mut`
    Mut,
    /// `pub`
    Pub,
    /// `return`
    Return,
    /// `true`
    True,
    /// `while`
    While,

    /// An error occurred during lexing.
    Error,
    /// End of input was reached.
    Eof,
}
