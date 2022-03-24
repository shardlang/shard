use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    // Keywords
    #[token("use")]
    Use,

    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("loop")]
    Loop,
    #[token("return")]
    Return,
    #[token("yield")]
    Yield,

    #[token("let")]
    Let,
    #[token("const")]
    Const,

    #[token("fn")]
    Fn,
    #[token("shard")]
    Shard,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("type")]
    Type,
    #[token("impl")]
    Impl,

    #[token("local")]
    Local,
    #[token("extern")]
    Extern,

    // Symbol tokens
    #[token("=")]
    Eq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[token("&")]
    Ampersand,
    #[token("|")]
    Bar,
    #[token("!")]
    Bang,

    #[token(".")]
    Period,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Ident,
    #[regex("[0-9]+")]
    Int,

    #[token("\n")]
    Newline,
    #[regex(r"[ \t\f]+")]
    Whitespace,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,

    Eof,
}
