/// A lexical token.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// A number.
    Number(f64),
    /// An identifier.
    Ident(String),
    /// A string literal.
    StaticString(String),
    /// A comment.
    Comment(String),
    /// A hashbang.
    Hashbang(String),
    /// The `+` operator.
    Add,
    /// The `-` operator.
    SubOrMinus,
    /// The `*` operator.
    Mul,
    /// The `/` operator.
    Div,
    /// The `%` operator.
    Mod,
    /// The `^` operator.
    Power,
    /// The `#` operator.
    Len,
    /// The `:` operator.
    Colon,
    /// The `::` operator.
    DoubleColon,
    /// The `;` operator.
    Semicolon,
    /// The `,` operator.
    Comma,
    /// The `=` operator.
    Assignment,
    /// The `==` operator.
    Equal,
    /// The `~=` operator.
    NotEqual,
    /// The `<` operator.
    LessThan,
    /// The `<=` operator.
    LessThanEqual,
    /// The `>` operator.
    GreaterThan,
    /// The `>=` operator.
    GreaterThanEqual,
    /// The `.` operator.
    MemberAccess,
    /// The `..` operator.
    Concat,
    /// The `...` operator.
    VarArgs,
    /// The `$` operator.
    Dollar,
    /// The `|` operator.
    Lambda,
    /// The `{` operator.
    OpenBrace,
    /// The `}` operator.
    CloseBrace,
    /// The `(` operator.
    OpenParen,
    /// The `)` operator.
    CloseParen,
    /// The `[` operator.
    OpenBracket,
    /// The `]` operator.
    CloseBracket,
}

/// Implements `From<&'a str>` for `Token`.
impl<'a> From<&'a str> for Token {
    fn from(val: &'a str) -> Token {
        Token::Ident(val.to_string())
    }
}