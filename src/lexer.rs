use crate::token::Token;
pub(crate) struct Lexer {
    tokens: Vec<Token>,
}
impl Lexer {
    pub(crate) fn new(input: String) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' => Token::Atom(c), //| 'a'..='z' | 'A'..='Z' if you want the power to  parser letters
                _ => Token::Op(c),
            })
            .collect::<Vec<Token>>();
        tokens.push(Token::Eof);
        tokens.reverse();
        Lexer { tokens }
    }

    pub(crate) fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }
    pub(crate) fn peek(&self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}
