use std::fmt;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::BangEqual => write!(f, "!="),
            TokenKind::Equal => write!(f, "="),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::AmpersandAmpersand => write!(f, "&&"),
            TokenKind::Bar => write!(f, "|"),
            TokenKind::BarBar => write!(f, "||"),
            TokenKind::Identifier => write!(
                f,
                "{}",
                self.literal.to_owned().expect("Identifier has no literal!")
            ),
            TokenKind::Str => write!(
                f,
                "{}",
                self.literal.to_owned().expect("String has no literal!")
            ),
            TokenKind::Num => write!(
                f,
                "{}",
                self.literal.to_owned().expect("Number has no literal!")
            ),
            TokenKind::Const => write!(f, "const"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Function => write!(f, "fn"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::For => write!(f, "for"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Print => write!(f, "print"),
            TokenKind::Whitespace => write!(f, " "),
            TokenKind::Comment => {
                write!(f, "//{}", self.literal.to_owned().unwrap_or("".to_owned()))
            }
            TokenKind::Eof => write!(f, ""),
        }
    }
}
