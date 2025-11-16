use std::fmt;
use crate::lexer::Lexer;
use crate::parser::parse_expression;

#[derive(Debug)]
pub enum Expression {
    Atom(char),
    Operation(char, Vec<Expression>),
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atom(i) => write!(f, "{}", i),
            Expression::Operation(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
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