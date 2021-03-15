use crate::lexer::{Token, TokenKind};
use crate::parser::Expression;

pub struct Parser {
    tokens: Vec<Token>,
    current: i64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Box<Expression> {
        self.equality_or_higher()
    }

    fn equality_or_higher(&mut self) -> Box<Expression> {
        let mut expr = self.comparison_or_higher();

        while self.match_token_kinds(vec![TokenKind::BangEqual, TokenKind::EqualEqual]) {
            expr = Box::new(Expression::new_binary(
                expr,
                self.previous().to_owned(),
                self.comparison_or_higher(),
            ));
        }

        expr
    }

    fn match_token_kinds(&mut self, token_kinds: Vec<TokenKind>) -> bool {
        for token_kind in token_kinds {
            if self.check(token_kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_kind: TokenKind) -> bool {
        if self.at_eof() {
            return false;
        }

        self.peek().kind == token_kind
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current as usize]
    }

    fn advance(&mut self) -> &Token {
        if self.at_eof() {
            self.current += 1;
        }
        self.previous()
    }

    fn at_eof(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current as usize - 1]
    }

    fn comparison_or_higher(&mut self) -> Box<Expression> {
        let mut expr = self.term_or_higher();

        while self.match_token_kinds(vec![
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            expr = Box::new(Expression::new_binary(
                expr,
                self.previous().to_owned(),
                self.term_or_higher(),
            ));
        }

        expr
    }

    fn term_or_higher(&mut self) -> Box<Expression> {
        let mut expr = self.factor_or_higher();

        while self.match_token_kinds(vec![TokenKind::Minus, TokenKind::Plus]) {
            expr = Box::new(Expression::new_binary(
                expr,
                self.previous().to_owned(),
                self.factor_or_higher(),
            ));
        }

        expr
    }

    fn factor_or_higher(&mut self) -> Box<Expression> {
        let mut expr = self.unary_or_higher();

        while self.match_token_kinds(vec![TokenKind::Slash, TokenKind::Star]) {
            expr = Box::new(Expression::new_binary(
                expr,
                self.previous().to_owned(),
                self.unary_or_higher(),
            ));
        }

        expr
    }

    fn unary_or_higher(&mut self) -> Box<Expression> {
        if self.match_token_kinds(vec![TokenKind::Bang, TokenKind::Minus]) {
            return Box::new(Expression::new_unary(
                self.previous().to_owned(),
                self.unary_or_higher(),
            ));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Expression> {
        let expr = if self.match_token_kinds(vec![
            TokenKind::False,
            TokenKind::True,
            TokenKind::Num,
            TokenKind::Str,
        ]) {
            Expression::new_literal(self.previous().to_owned())
        } else if self.match_token_kinds(vec![TokenKind::LeftParen]) {
            let inner = self.expression();
            self.consume(TokenKind::RightParen, "Expect ')' after expression");
            Expression::new_grouping(inner)
        } else {
            //error
            Expression::new_literal(self.previous().to_owned())
        };

        Box::new(expr)
    }

    fn consume(&self, token_kind: TokenKind, expect_msg: &str) {
        unimplemented!();
    }
}
