use crate::token::Token;
use crate::lexer::Lexer;
use crate::expression::Expression;

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
pub fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression {
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