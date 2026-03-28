use crate::lexer::Token;
use colored::Colorize;

mod lexer;
mod parser;

fn main() {
    let input: String = String::from("  33.5  + 4.3 +  5.2");
    let mut lexer = lexer::Lexer::new(input);

    let mut result: Vec<Token> = Vec::new();

    loop {
        let token = lexer.next_token();
        match token {
            Ok(token) => {
                result.push(token);
                println!("{:?}", token);
                if token == lexer::Token::EOO {
                    break;
                }
            }
            Err(e) => {
                println!("{}", format!("LEXER ERROR: {}", e).red().bold());
                return;
            }
        }
    }

    let mut parser = parser::Parser::new(result);
    let result = parser.parse();
    match result {
        Ok(tree) => {
            println!("Result: {:?}", tree);
        }
        Err(e) => {
            println!("{}", format!("PARSER ERROR: {}", e).red().bold());
        }
    }
}
