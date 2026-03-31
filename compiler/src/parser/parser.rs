use std::string;

use crate::{
    lexer::token::{DataType, Token, Data},
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

    fn cut(&mut self) {
        self.tokens.remove(self.pos);
    }

    pub fn parse_variable_declaration(&mut self) -> Result<Variable, String> {
        // Identifies "let" keyword
        match self.current() {
            Some(Token::LET) => {}
            _ => return Err("Missing Variable Declaration".into()),
        }

        // Gets the variable name
        let name = match self.advance() {
            Some(Token::IDENT(n)) => n,
            None => return Err("Missing Variable Name".into()),
            _ => return Err("Unexpected Variable Name".into()),
        };
        let name: String = name.iter().collect();

        let mut data_type_before: Option<DataType> = None;

        if self.peek().unwrap().clone() == Token::COLON {
            self.advance();
            data_type_before = match self.advance().unwrap().clone() {
                Token::DATATYPE(dt) => Some(dt),
                _ => return Err(format!("Unexpected Data Type, got: {:?}", self.current().unwrap().clone())),
            };
            println!("{:?}", data_type_before);
        }


        // Gets the variable data type (optional, lexer guesses for you)
        let (mut data_type, mut value) = match self.advance().unwrap().clone() {
            Token::ASSIGN => {
                let value = self.advance().unwrap().clone();
                match value {
                    Token::DATA(Data::INT(v)) => (DataType::INT, Value::INT(v)),
                    Token::DATA(Data::FLOAT(v)) => (DataType::FLOAT, Value::FLOAT(v)),
                    Token::DATA(Data::STRING(v)) => (DataType::STRING, Value::STRING(v)),
                    Token::DATA(Data::BOOL(v)) => (DataType::BOOL, Value::BOOL(v)),
                    _ => panic!("Idk, I'm just not bothered for error handling right now. Find this message in the code and figure it out yourself")
                }
            }
            _ => unreachable!()
        };

        if data_type_before.is_some() {
            if data_type_before.clone().unwrap() != data_type {
                if data_type_before.clone().unwrap() == DataType::FLOAT && data_type == DataType::INT {
                    data_type = DataType::FLOAT;
                    if let Value::INT(v) = value {
                        value = Value::FLOAT(v as f64);
                    }
                } else if data_type_before.clone().unwrap() == DataType::INT && data_type == DataType::FLOAT {
                    data_type = DataType::INT;
                    if let Value::FLOAT(v) = value {
                        value = Value::INT(v as i64);
                    }
                } else {
                    return Err(format!("Data type mismatch: expected {:?}, got {:?}", data_type_before.unwrap(), data_type));
                }
            }
        }

        self.advance();
        match self.expect(vec![Token::SEMICOLON]) {
            Ok(_) => Ok(Variable {
                name: Name { name: name },
                value: value,
                data_type: data_type,
            }),
            Err(e) => return Err("Incorect variable declaration: missing semicolon".into()),
        }
    }

    fn expect(&mut self, expected: Vec<Token>) -> Result<Token, String> {
        for token in &expected {
            if *self.current().unwrap() == *token {
                return Ok(token.clone());
            }
        }
        return Err(format!(
            "Expected {:?}, found {:?}",
            expected,
            self.current().unwrap()
        ));
    }
}
