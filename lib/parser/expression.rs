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
            Expression::Unary { op, right } => write!(f, "({0} {1})", op, right.to_string()),
            Expression::Binary { left, op, right } => {
                write!(f, "({0} {1} {2})", left.to_string(), op, right.to_string())
            }
            Expression::Grouping { expr } => write!(f, "({})", expr.to_string()),
            Expression::Literal { lit } => write!(f, "({})", lit),
        }
    }
}
