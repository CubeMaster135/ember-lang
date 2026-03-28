use crate::lexer::Token;

mod lexer;
mod parser;

fn main() {
    let input: String = String::from("4 * (3 + 5) / 2");
    let mut lexer = lexer::Lexer::new(input);

    let mut result: Vec<Token> = Vec::new();

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        result.push(token);
        if token == lexer::Token::EOO {
            break;
        }
    }

    let mut parser = parser::Parser::new(result);
    let result = parser.parse();
    println!("Result: {:?}", parser.tree());
}
