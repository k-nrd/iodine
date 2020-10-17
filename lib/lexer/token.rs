#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: Option<String>,
    pub line: u32,
    pub col: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
    Ampersand,
    AmpersandAmpersand,
    Bar,
    BarBar,

    // Literals.
    Identifier,
    Str,
    Num,

    // Keywords
    Const,
    Let,
    True,
    False,
    Function,
    Return,
    For,
    If,
    Else,
    Print,

    Whitespace,
    Comment,
    Eof,
}
