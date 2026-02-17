use std::fmt;
use crate::parser::*;
use crate::lexer::Lexer;

/*
    Reasons for this implementation:
    The use of enum is a more elegant way to manage the tree.
    If you want to implement it by struct and you have to use Empty nodes you should use option that are enums.

    Type of three:
    At the moment is perfectly binary, there are only binary operation.
*/
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Error(String),
    Zero,
    Atom(i128),
    Operation(char, Box<Expression>, Box<Expression>),
}

//Display Expression in prefix notation
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Error(str)=>write!(f,"{}",str),
            Expression::Zero=>write!(f, ""),
            Expression::Atom(i) => write!(f, "{}", i),
            Expression::Operation(op, expr1, expr2) => {
                write!(f, "({}", op)?;
                write!(f, " {}", expr1)?;
                write!(f, " {}", expr2)?;
                write!(f, ")")
            }
        }
    }
}

//Parser caller
impl Expression {
    pub fn parse_from_prefix_to_expression(tokens: &mut Vec<&str>)->Expression{
       let token = tokens.pop().unwrap();
       if token==""{
            return Expression::Zero;
       }
       if token == "+" || token == "-" || token == "*" || token == "/" || token == "^"{
            return Expression::Operation(token.chars().next().unwrap(), Box::new(Expression::parse_from_prefix_to_expression(tokens)), Box::new(Expression::parse_from_prefix_to_expression(tokens)));
       }
       let number = token.parse::<i128>().unwrap();
       return Expression::Atom(number);
    }

    //Convert the str expression in un AST
    pub(crate) fn convert_token_to_ast(mut input: Lexer) -> Expression {
        match parse_expression_with_parethesis(&mut input, 0.0){
            //when a error occurs save the error in a Error enum in order to signal the problem to the user
            Err(str)=>Expression::Error(str), 
            Ok(expr)=> expr,
        }
    }
}

//Some print methods
impl Expression {
    pub fn print_prefix(&self)->String{
        format!("{}", self)
    }
    pub fn print_infix(&self){
        self.print_infix1();
        println!("");
    }
    pub fn print_infix1(&self){
        match self {
            Expression::Error(str)=>{
                print!("{}", str);
            },
            Expression::Zero=>{}
            Expression::Atom(value) => {
                print!("{}", value);
            }
            Expression::Operation(op, left, right) => {
                left.print_infix1();
                print!("{}", op);
                right.print_infix1();
                // print!("{}", op);
            }
        }
    }
    pub fn printree(&self, prefix: &str, last: bool) {
        match self {
            Expression::Error(str)=>{
                print!("{}", str);
            },
            Expression::Zero=>{},
            Expression::Atom(value) => {
                println!("{}{}{}", prefix, if last { "└── " } else { "├── " }, value);
            },
            Expression::Operation(value, left, right) => {
                println!("{}{}{}", prefix, if last { "└── " } else { "├── " }, value);
                let new_prefix = format!("{}{}", prefix, if last { "    " } else { "│   " });
                right.printree(&new_prefix, false);
                left.printree(&new_prefix, true);
            },
        }
    }
    /// Generates the visual representation of the AST tree using ASCII characters.
    pub fn print_visual(&self) -> String {
        // The recursive printing function uses a Vector to build the output.
        let mut output = Vec::new();

        // Start the recursion: The root is not considered a left child (false)
        self.print_recursive("", false, &mut output);

        output.join("\n")
    }

    /// Recursive helper function for visual tree printing.
    ///
    /// `prefix`: The accumulated indentation string (vertical lines and spaces).
    /// `is_left`: Indicates if the CURRENT node is the left child of its parent.
    fn print_recursive(&self, prefix: &str, is_left: bool, output: &mut Vec<String>) {
        match self {
            Expression::Error(str)=>{
                print!("{}", str);
            },
            Expression::Zero=>{}
            Expression::Operation(op, left, right) => {
                // 1. Print the RIGHT branch (displayed at the top)
                // If the CURRENT node is the left child (`is_left`), the line must continue (│)
                // If the CURRENT node is not the left child, the line is a blank space ( )
                right.print_recursive(
                    &(prefix.to_owned() + if is_left { "│   " } else { "    " }),
                    false, // The right child is not the left child
                    output,
                );

                // 2. Print the CURRENT node
                // The connector is '└── ' if it is the left child, '┌── ' otherwise.
                output.push(format!(
                    "{}{} {}",
                    prefix,
                    if is_left { "└──" } else { "┌──" },
                    op
                ));

                // 3. Print the LEFT branch (displayed at the bottom)
                // If the CURRENT node is the left child, the line is a blank space (the parent branch ends here)
                // If the CURRENT node is not the left child, the line must continue (│)
                left.print_recursive(
                    &(prefix.to_owned() + if is_left { "    " } else { "│   " }),
                    true, // The left child is the left child
                    output,
                );
            }

            Expression::Atom(c) => {
                // Base Case: Print the leaf
                output.push(format!(
                    "{}{} {}",
                    prefix,
                    if is_left { "└──" } else { "┌──" },
                    c
                ));
            }
        }
    }
}
