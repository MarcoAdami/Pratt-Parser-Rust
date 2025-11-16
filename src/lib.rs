use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Token {
    Atom(char),
    Op(char),
    Eof,
}
struct Lexer {
    tokens: Vec<Token>,
}
impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c),
            })
            .collect::<Vec<Token>>();
        tokens.push(Token::Eof);
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }
    fn peek(&self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

fn infix_binding_power(op: char) -> (f32, f32) {
    match op {
        '=' => (0.1, 0.2),
        '+' => (1.0, 1.1),
        '-' => (1.2, 1.3),
        '*' | '/' => (2.0, 2.1),
        '^' | '√' => (3.1, 3.0),
        _ => panic!("unknown operator: {:?}", op),
    }
}

/*
   We scan the expression iterating through it,
   but there could be some problem like the rhs operands could
   have a stronger binding power the lhs so we have to account
   for that iterating a recursing through it before merging it
*/
fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression {
    //left hand side of the operand
    let mut lhs = match lexer.next() {
        Token::Atom(it) => Expression::Atom(it),
        Token::Op('(') => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        t => panic!("bad token: {:?}", t),
        // Token::Op(_) | Token::Eof => todo!(),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof | Token::Op(')') => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp {
            break;
        }
        lexer.next();
        let rhs = parse_expression(lexer, r_bp);
        lhs = Expression::Operation(op, vec![lhs, rhs]);
    }
    lhs
}

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

// fn main() {
//     let exp = Expression::from_str("1+2+3");
//     println!("Result: {}", exp.to_string());
// }

// #[cfg(test)]
// mod tests {}
