use crate::lexer::Lexer;
use crate::parser::parse_expression;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Atom(char),
    Operation(char, Box<Expression>, Box<Expression>),
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atom(i) => write!(f, "{}", i),
            Expression::Operation(head, son1, son2) => {
                write!(f, "({}", head)?;
                write!(f, " {}", son1)?;
                write!(f, " {}", son2)?;
                write!(f, ")")
            }
        }
    }
}

impl Expression {
    pub fn from_str(input: &str) -> Expression {
        let mut lexer = Lexer::new(input);
        parse_expression(&mut lexer, 0.0)
    }
}
