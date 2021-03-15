use crate::lexer::Token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Unary {
        op: Token,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },
    Grouping {
        expr: Box<Expression>,
    },
    Literal {
        lit: Token,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Unary { op, right } => write!(f, "{0}{1}", op, right.to_string()),
            Expression::Binary { left, op, right } => {
                write!(f, "{0} {1} {2}", left.to_string(), op, right.to_string())
            }
            Expression::Grouping { expr } => write!(f, "({})", expr.to_string()),
            Expression::Literal { lit } => write!(f, "{}", lit),
        }
    }
}

impl Expression {
    pub fn new_unary(op: Token, right: Box<Expression>) -> Expression {
        Expression::Unary { op, right }
    }
    pub fn new_binary(left: Box<Expression>, op: Token, right: Box<Expression>) -> Expression {
        Expression::Binary { left, op, right }
    }

    pub fn new_grouping(expr: Box<Expression>) -> Expression {
        Expression::Grouping { expr }
    }

    pub fn new_literal(lit: Token) -> Expression {
        Expression::Literal { lit }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::TokenKind;

    #[test]
    fn expression_prints() {
        let expr = Expression::Unary {
            op: Token {
                kind: TokenKind::Minus,
                col: 1,
                line: 1,
                literal: Some("-".to_string()),
            },
            right: Box::new(Expression::Literal {
                lit: Token {
                    kind: TokenKind::Num,
                    col: 2,
                    line: 1,
                    literal: Some("1".to_string()),
                },
            }),
        };
        println!("{}", expr.to_string());
        assert_eq!(expr.to_string(), "-1".to_string());
    }
}
