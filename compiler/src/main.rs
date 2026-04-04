mod lexer;
mod parser;
use crate::parser::parser::*;
use std::fs::read_to_string;

fn main() -> Result<(), String> {
    let file = read_to_string("code/main.txt").unwrap();
    let input = String::from(file);
    let mut lexer_output = Vec::<lexer::token::Token>::new();
    let mut l = lexer::Lexer::new(input.chars().collect());
    l.read_char();
    loop {
        let token = l.next_token();
        lexer_output.push(token.clone());
        println!("{:?}", token);
        if token == lexer::token::Token::EOF {
            break;
        }
    }
    let mut parser = Parser::new(lexer_output);
    println!("{:?}", parser.parse_variable_declaration()?);
    Ok(())
}
