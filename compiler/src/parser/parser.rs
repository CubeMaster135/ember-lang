use std::string;

use crate::{
    lexer::token::{DataType, Token},
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

    fn figure_out_data_type(&mut self, ident: Token) -> Result<DataType, String> {
        match ident {
            Token::SMARK => {
                self.cut();
                self.pos += 1;
                if self.current().unwrap() == &Token::SMARK {
                    self.cut();
                } else {
                    panic!("Expecting closing speach mark");
                }
                self.pos -= 1;
                Ok(DataType::STRING)
            }
            Token::IDENT(s) => {
                if s.iter().collect::<String>().parse::<i32>().is_ok() {
                    return Ok(DataType::INT);
                }
                if s.iter().collect::<String>().parse::<f32>().is_ok() {
                    return Ok(DataType::FLOAT);
                }
                Err(String::from("Invalid data type"))
            }
            Token::TRUE => Ok(DataType::BOOL),
            Token::FALSE => Ok(DataType::BOOL),
            Token::INT(_) => Ok(DataType::INT),
            _ => Err(format!(
                "Didn't receive identifer token, recieved: {:?}",
                ident,
            )),
        }
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

        // Gets the variable data type (optional, defaults to string)
        let data_type: DataType = match self.advance() {
            Some(Token::COLON) => {
                self.advance(); // move to the datatype token
                let dt = match self.current() {
                    Some(Token::DATATYPE(dt)) => dt.clone(),
                    _ => return Err("Unexpected Data Type".into()),
                };
                self.advance(); // move to ASSIGN
                dt
            }
            Some(Token::ASSIGN) => {
                self.advance(); // move to the value token
                let dt = self.figure_out_data_type(self.current().unwrap().clone());
                self.pos -= 1;
                dt.unwrap()
            }
            _ => {
                return Err(format!(
                    "Expected ':' or '=' after variable name, got: {:?}",
                    self.current()
                ));
            }
        };

        // now self.current() should be ASSIGN in both branches
        match self.current() {
            Some(Token::ASSIGN) => {}
            _ => {
                return Err(format!(
                    "Expected '=' before variable value, got: {:?}",
                    self.current()
                ));
            }
        }

        let value: Value = match data_type {
            DataType::INT => {
                let v = self.advance().unwrap();
                match v {
                    Token::INT(v) => {
                        let value = v.into_iter().collect::<String>().parse::<i64>().unwrap();
                        Value::INT(value)
                    }
                    _ => {
                        return Err(format!(
                            "Unexpected Variable Value: {:?}",
                            self.current().unwrap()
                        ));
                    }
                }
            }
            DataType::STRING => {
                let v = self.advance().unwrap();
                match v {
                    Token::IDENT(v) => {
                        let value = v.into_iter().collect::<String>();
                        Value::STRING(value)
                    }
                    _ => {
                        return Err(format!(
                            "Unexpected Variable Value: {:?}",
                            self.current().unwrap()
                        ));
                    }
                }
            }
            DataType::BOOL => {
                let v = self.advance().unwrap();
                match v {
                    Token::TRUE => Value::BOOL(true),
                    Token::FALSE => Value::BOOL(false),
                    _ => {
                        return Err(format!(
                            "Unexpected Variable Value: {:?}",
                            self.current().unwrap()
                        ));
                    }
                }
            }
            DataType::FLOAT => {
                let v = self.advance().unwrap();
                match v {
                    Token::INT(v) => {
                        let value = v.into_iter().collect::<String>().parse::<f64>().unwrap();
                        Value::FLOAT(value)
                    }
                    _ => {
                        return Err(format!(
                            "Unexpected Variable Value: {:?}",
                            self.current().unwrap()
                        ));
                    }
                }
            }
        };
        self.advance();
        match self.expect(vec![Token::SEMICOLON]) {
            Ok(_) => Ok(Variable {
                name: Name { name: name },
                value: value,
                data_type: data_type,
            }),
            Err(e) => return Err("Incorect variable declaration".into()),
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
