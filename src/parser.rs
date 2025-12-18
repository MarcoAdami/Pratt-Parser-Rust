use crate::costum_result::MyResult;
use crate::expression::Expression;
use crate::lexer::Lexer;
use crate::token::Token;

fn infix_binding_power(op: char) -> (f32, f32) {
    match op {
        '=' => (0.1, 0.2),
        '+'|'-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
        '^' => (3.1, 3.0),
        _ => panic!("unknown operator: {:?}", op),
    }
}

/*
   We scan the expression iterating through it,
   but there could be some problem like the rhs operands could
   have a stronger binding power the lhs so we have to account
   for that iterating a recursing through it before merging it
*/
pub fn parse_expression(lexer: &mut Lexer, min_bp: f32, nested: &mut u8) -> MyResult {
    //left hand side of the operand
    let mut lhs;
    match lexer.next() {
        Token::Atom(it) => {
            let mut temp: i128 = it.to_digit(10).unwrap_or(0) as i128;
            loop {
                match lexer.peek() {
                    Token::Atom(num) => temp = temp * 10 + num.to_digit(10).unwrap_or(0) as i128,
                    _ => break,
                };
                lexer.next();
            }
            lhs = Expression::Atom(temp);
        }
        Token::Op('(') => {
            let result = parse_expression(lexer, 0.0, &mut (*nested + 1));
            match &result.0 {
                Ok(expr) => lhs = expr.clone(),
                Err(msg) => {
                    return MyResult {
                        0: Err(format!("{}", msg)),
                    };
                }
            };
            match lexer.next() {
                Token::Op(')') => {}
                t => return MyResult(Err(format!("Expected ')', got {:?}", t))),
            }
        }
        t => {
            return MyResult {
                0: Err(format!("bad token: {:?}", t)),
            };
        }
    };
    // println!("{}", lhs);

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(')') => {
                // lexer.next();
                // println!("finish");
                break;
            }
            Token::Op(op) => op,
            t => {
                return MyResult {
                    0: Err(format!("bad token: {:?}", t)),
                };
            }
        };
        // println!("{}", op);

        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp {
            break;
        }
        lexer.next();
        let rhs = match parse_expression(lexer, r_bp, nested).0 {
            Ok(expr) => expr,
            Err(msg) => return MyResult { 0: Err(msg) },
        };
        // println!("{}", rhs);
        lhs = Expression::Operation(op, Box::new(lhs), Box::new(rhs));
    }
    MyResult { 0: Ok(lhs) }
}
