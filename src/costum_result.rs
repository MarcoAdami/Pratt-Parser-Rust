use crate::expression::Expression;
use std::fmt;

pub struct MyResult(pub Result<Expression, String>);

impl fmt::Display for MyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Ok(expr) => write!(f, "{}", expr),
            Err(msg) => write!(f, "{}", msg),
        }
    }
}

impl MyResult {
    pub fn to_string(&self) -> String {
        match &self.0 {
            Ok(expr) => format!("{}", expr),
            Err(msg) => msg.clone(),
        }
    }

    pub fn printree(&self, prefix: &str, last: bool) {
        match &self.0 {
            Ok(expr) => expr.printree(prefix, last),
            Err(msg) => println!("Not well formed expression: {}", msg),
        }
    }
    pub fn print_visual(&self) -> String {
        match &self.0 {
            Ok(expr) => expr.print_visual(),
            Err(msg) => format!("Not well formed expression: {}", msg),
        }
    }
}
