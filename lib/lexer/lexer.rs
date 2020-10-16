use super::{LexerError, ParseTokenError, Token, TokenKind};

pub struct Lexer<'a> {
    source: &'a [u8],
    start: usize,
    current: usize,
    line: u32,
    col: u32,
    swap: Option<String>,
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
        self.current += 1;
        self.source[self.current - 1]
    }

    fn peek(&mut self) -> u8 {
        if self.at_eof() {
            return b'\0';
        }
        self.source[self.current]
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
        if self.peek() != b'/' {
            return TokenKind::Slash;
        }
        while self.peek() != b'\n' && !self.at_eof() {
            self.advance();
        }
        self.line += 1;
        TokenKind::Comment
    }

    fn eat_whitespace(&mut self, newline: bool) -> TokenKind {
        if newline {
            self.line += 1;
        }
        while self.peek().is_ascii_whitespace() && !self.at_eof() {
            if self.advance() == b'\n' {
                self.line += 1;
                self.col = 1;
            }
        }
        TokenKind::Whitespace
    }

    fn eat_string(&mut self) -> Result<TokenKind, ParseTokenError> {
        while self.peek() != b'"' && !self.at_eof() {
            if self.peek() == b'\n' {
                self.line += 1;
                self.col = 1;
            }
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
            b'/' => self.check_comment(),
            b'\n' => self.eat_whitespace(true),
            b' ' => self.eat_whitespace(false),
            b'\r' => self.eat_whitespace(false),
            b'\t' => self.eat_whitespace(false),
            b'"' => self.eat_string()?,
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
