use crate::{
    lexer::token::Token,
    parser::{Name, Value, Variable},
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn position(&self) -> usize {
        self.pos
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn advance(&mut self) -> Option<&Token> {
        self.pos += 1;
        self.tokens.get(self.pos)
    }

    pub fn parse_variable_declaration(&mut self) -> Result<Variable, String> {
        match self.current() {
            Some(Token::LET) => {}
            _ => return Err("Missing Variable Declaration".into()),
        }
        let name = match self.advance().unwrap() {
            Token::IDENT(n) => n,
            _ => return Err("Unexpected Variable Name".into()),
        };
        let name: String = name.iter().collect();
        self.advance();
        self.expect(Token::ASSIGN)?;
        let value = match self.advance().unwrap() {
            Token::INT(v) => v,
            _ => return Err("Unexpected Variable Value".into()),
        };
        let value: String = value.into_iter().collect();
        self.advance();
        match self.expect(Token::SEMICOLON) {
            Ok(()) => Ok(Variable {
                name: Name { name: name },
                value: Value { value },
            }),
            Err(e) => return Err("Missing semicolon".into()),
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if *self.current().unwrap() == expected {
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                expected,
                self.current().unwrap()
            ))
        }
    }
}
