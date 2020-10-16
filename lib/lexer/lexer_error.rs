use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct ParseTokenError {
    pub line: u32,
    pub col: u32,
    pub text: String,
}

impl ParseTokenError {
    pub fn new(line: u32, col: u32, text: String) -> Self {
        ParseTokenError { line, col, text }
    }
}

impl Display for ParseTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error parsing token \"{0}\" at line {1}, column {2}.",
            self.text, self.line, self.col
        )
    }
}

impl Error for ParseTokenError {}

#[derive(Debug)]
pub struct LexerError {
    errors: Vec<ParseTokenError>,
}

impl LexerError {
    pub fn new() -> Self {
        LexerError { errors: vec![] }
    }

    pub fn add(&mut self, err: ParseTokenError) {
        self.errors.push(err);
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut err_strings: Vec<String> = vec![];
        for err in &self.errors {
            err_strings.push(format!("{}", err));
        }
        write!(f, "Error parsing tokens: \n{}", err_strings.join("\n"))
    }
}

impl Error for LexerError {}
