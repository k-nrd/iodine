use super::{LexerError, ParseTokenError, Token, TokenKind};
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "const" => TokenKind::Const,
    "let" => TokenKind::Let,
    "true" => TokenKind::True,
    "false" => TokenKind::False,
    "fn" => TokenKind::Function,
    "return" => TokenKind::Return,
    "for" => TokenKind::For,
    "if" => TokenKind::If,
    "else" => TokenKind::Else,
    "print" => TokenKind::Print,
};

pub struct Lexer<'a> {
    source: &'a [u8],
    start: usize,
    current: usize,
    line: u32,
    col: u32,
    swap: Option<String>,
}

fn is_identifier_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_identifier_char(b: u8) -> bool {
    is_identifier_start(b) || is_digit(b)
}

fn is_digit(b: u8) -> bool {
    b.is_ascii_digit()
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Lexer {
            start: 0,
            current: 0,
            line: 1,
            col: 1,
            source,
            swap: None,
        }
    }

    fn token(&mut self, kind: TokenKind, literal: Option<String>) -> Token {
        Token {
            kind,
            literal,
            line: self.line,
            col: self.col,
        }
    }

    fn parse_error(&mut self, text: &str) -> ParseTokenError {
        ParseTokenError {
            line: self.line,
            col: self.col,
            text: text.to_owned(),
        }
    }

    fn advance(&mut self) -> u8 {
        if self.peek_n(1) == b'\n' {
            self.line += 1;
            self.col = 1;
        }
        self.current += 1;
        self.source[self.current - 1]
    }

    fn peek_n(&mut self, lookahead: usize) -> u8 {
        let index = self.current + lookahead - 1;
        if index >= self.source.len() {
            return b'\0';
        }
        self.source[index]
    }

    fn at_eof(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn check_double(&mut self, expected: u8, double: TokenKind, single: TokenKind) -> TokenKind {
        if self.at_eof() {
            return single;
        }
        if self.source[self.current] != expected {
            return single;
        }
        self.current += 1;
        double
    }

    fn check_comment(&mut self) -> TokenKind {
        if self.peek_n(1) != b'/' {
            return TokenKind::Slash;
        }
        while self.peek_n(1) != b'\n' && !self.at_eof() {
            self.advance();
        }
        TokenKind::Comment
    }

    fn eat_whitespace(&mut self, newline: bool) -> TokenKind {
        if newline {
            self.line += 1;
        }
        while self.peek_n(1).is_ascii_whitespace() && !self.at_eof() {
            self.advance();
        }
        TokenKind::Whitespace
    }

    fn eat_string(&mut self) -> Result<TokenKind, ParseTokenError> {
        while self.peek_n(1) != b'"' && !self.at_eof() {
            self.advance();
        }

        if self.at_eof() {
            return Err(self.parse_error("Unterminated string!"));
        }

        self.advance();

        if let Some(lit_bytes) = self.source.get(self.start + 1..self.current - 1) {
            if let Ok(lit_string) = String::from_utf8(lit_bytes.to_vec()) {
                self.swap = Some(lit_string);
                return Ok(TokenKind::Str);
            }
        }

        Err(self.parse_error("Error parsing string!"))
    }

    fn eat_number(&mut self) -> Result<TokenKind, ParseTokenError> {
        while is_digit(self.peek_n(1)) {
            self.advance();
        }

        if self.peek_n(1) == b'.' && is_digit(self.peek_n(2)) {
            self.advance();

            while is_digit(self.peek_n(1)) {
                self.advance();
            }
        }

        if let Some(lit_bytes) = self.source.get(self.start..self.current) {
            if let Ok(lit_string) = String::from_utf8(lit_bytes.to_vec()) {
                self.swap = Some(lit_string);
                return Ok(TokenKind::Num);
            }
        }

        Err(self.parse_error("Error parsing number!"))
    }

    fn eat_identifier(&mut self) -> Result<TokenKind, ParseTokenError> {
        while is_identifier_char(self.peek_n(1)) {
            self.advance();
        }

        if let Some(lit_bytes) = self.source.get(self.start..self.current) {
            if let Ok(lit_string) = String::from_utf8(lit_bytes.to_vec()) {
                let token_kind;
                if let Some(keyword_kind) = KEYWORDS.get(lit_string.as_str()) {
                    token_kind = keyword_kind.to_owned();
                } else {
                    token_kind = TokenKind::Identifier;
                }
                self.swap = Some(lit_string);
                return Ok(token_kind);
            }
        }

        Err(self.parse_error("Error parsing identifier!"))
    }

    fn start_match(&mut self) -> Result<Token, ParseTokenError> {
        let b = self.advance();
        let token_kind = match b {
            b'(' => TokenKind::LeftParen,
            b')' => TokenKind::RightParen,
            b'{' => TokenKind::LeftBrace,
            b'}' => TokenKind::RightParen,
            b',' => TokenKind::Comma,
            b'.' => TokenKind::Dot,
            b'-' => TokenKind::Minus,
            b'+' => TokenKind::Plus,
            b';' => TokenKind::Semicolon,
            b'*' => TokenKind::Star,
            b'!' => self.check_double(b'=', TokenKind::BangEqual, TokenKind::Bang),
            b'=' => self.check_double(b'=', TokenKind::EqualEqual, TokenKind::Equal),
            b'>' => self.check_double(b'=', TokenKind::GreaterEqual, TokenKind::Greater),
            b'<' => self.check_double(b'=', TokenKind::LessEqual, TokenKind::Greater),
            b'&' => self.check_double(b'&', TokenKind::AmpersandAmpersand, TokenKind::Ampersand),
            b'|' => self.check_double(b'|', TokenKind::BarBar, TokenKind::Bar),
            b'/' => self.check_comment(),
            b'\n' => self.eat_whitespace(true),
            b' ' => self.eat_whitespace(false),
            b'\r' => self.eat_whitespace(false),
            b'\t' => self.eat_whitespace(false),
            b'"' => self.eat_string()?,
            d if is_digit(d) => self.eat_number()?,
            a if is_identifier_start(a) => self.eat_identifier()?,
            e => {
                return Err(self.parse_error(&format!(
                    "Error parsing unknown character {}",
                    String::from_utf8(vec![e]).unwrap()
                )))
            }
        };

        Ok(self.token(token_kind, self.swap.to_owned()))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = vec![];
        let mut errors: LexerError = LexerError::new();
        let mut has_err = false;

        while !self.at_eof() {
            self.start = self.current;
            self.col = self.start as u32 + 1;
            self.swap = None;

            match self.start_match() {
                Ok(tok) => tokens.push(tok),
                Err(lex_err) => {
                    has_err = true;
                    errors.add(lex_err);
                }
            };
        }

        tokens.push(self.token(TokenKind::Eof, None));

        if has_err {
            return Err(errors);
        }

        Ok(tokens)
    }
}
