#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: Option<String>,
    pub line: u32,
    pub col: u32,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Single character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One- or two-character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    Str,
    Num,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Const,
    Let,
    While,

    Whitespace,
    Comment,
    Eof,
}
