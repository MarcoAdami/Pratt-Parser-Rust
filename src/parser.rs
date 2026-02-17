use crate::expression::Expression;
use crate::lexer::Lexer;
use crate::token::Token;

fn infix_binding_power(op: char) -> (f32, f32) {
    match op {
        '=' => (0.1, 0.2),
        '+' | '-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
        '^' => (3.1, 3.0),
        _ => panic!("unknown operator: {:?}", op),
    }
}

pub(crate) fn parse_expression_with_parethesis(
    lexer: &mut Lexer,
    min_bp: f32,
) -> Result<Expression, String> {
    // Parse prefix (operando sinistro)
    let mut lhs = match lexer.next() {
        Token::Atom(it) => {
            let mut num = it.to_digit(10).unwrap() as i128;
            while let Token::Atom(digit) = lexer.peek() {
                num = num * 10 + digit.to_digit(10).unwrap() as i128;
                lexer.next();
            }
            Expression::Atom(num)
        }
        Token::Op('(') => {
            let result = parse_expression_with_parethesis(lexer, 0.0)?;
            let next_token = lexer.next();
            // CHECK at the end of the expression there should be the closing prarenthesis that has been opened
            if next_token != Token::Op(')') {
                return Err(format!("Expected ')', got {:?}", next_token));
            }
            result
        }
        t => return Err(format!("bad token: {:?}", t)),
    };

    // Parse operatori infissi
    loop {
        let op = match lexer.peek() {
            Token::Eof | Token::Op(')') => break,
            Token::Op(op) => op,
            t => return Err(format!("bad token: {:?}", t)),
        };

        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp {
            break;
        }

        lexer.next();
        let rhs = parse_expression_with_parethesis(lexer, r_bp)?;

        // Costruisci stringa prefix direttamente: "op lhs rhs"
        lhs = Expression::Operation(op, Box::new(lhs), Box::new(rhs));
    }

    Ok(lhs)
}
