use crate::{expression::Expression, lexer::Lexer};

pub fn convert_str_to_ast(str: String) -> Expression {
    let tokens = Lexer::new(str.to_string());
    Expression::convert_token_to_ast(tokens)
}
