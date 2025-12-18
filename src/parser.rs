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

pub fn parse_to_prefix(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    parse_expression(&mut lexer, 0.0)
}

pub fn parse_to_prefix_with_parenthesis(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    parse_expression_with_parethesis(&mut lexer, 0.0)
}

fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Result<String, String> {
    // Parse prefix (operando sinistro)
    let mut lhs = match lexer.next() {
        Token::Atom(it) => {
            let mut num = it.to_digit(10).unwrap() as i128;
            while let Token::Atom(digit) = lexer.peek() {
                num = num * 10 + digit.to_digit(10).unwrap() as i128;
                lexer.next();
            }
            num.to_string() // Stringa direttamente
        }
        Token::Op('(') => {
            let result = parse_expression(lexer, 0.0)?;
            match lexer.next() {
                Token::Op(')') => result,
                t => return Err(format!("Expected ')', got {:?}", t)),
            }
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
        let rhs = parse_expression(lexer, r_bp)?;
        
        // Costruisci stringa prefix direttamente: "op lhs rhs"
        lhs = format!("{} {} {}", op, lhs, rhs);
    }
    
    Ok(lhs)
}


fn parse_expression_with_parethesis(lexer: &mut Lexer, min_bp: f32) -> Result<String, String> {
    // Parse prefix (operando sinistro)
    let mut lhs = match lexer.next() {
        Token::Atom(it) => {
            let mut num = it.to_digit(10).unwrap() as i128;
            while let Token::Atom(digit) = lexer.peek() {
                num = num * 10 + digit.to_digit(10).unwrap() as i128;
                lexer.next();
            }
            num.to_string() // Stringa direttamente
        }
        Token::Op('(') => {
            let result = parse_expression_with_parethesis(lexer, 0.0)?;
            match lexer.next() {
                Token::Op(')') => result,
                t => return Err(format!("Expected ')', got {:?}", t)),
            }
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
        lhs = format!("({} {} {})", op, lhs, rhs);
    }
    
    Ok(lhs)
}