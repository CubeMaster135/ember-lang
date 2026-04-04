use crate::lexer::token::*;
use crate::parser::parser::*;
use crate::parser::*;

impl Parser {
    pub fn parse_variable_binding(&mut self) -> Result<VariableBinding, String> {
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
                _ => {
                    return Err(format!(
                        "Unexpected Data Type, got: {:?}",
                        self.current().unwrap().clone()
                    ));
                }
            };
            println!("{:?}", data_type_before);
        }

        // Gets the variable data type (optional, lexer guesses for you)
        let (mut data_type, value) = match self.advance().unwrap().clone() {
            Token::ASSIGN => {
                let value = self.advance().unwrap().clone();
                match value {
                    Token::DATA(Data::INT(v)) => (DataType::INT, Value::INT(v)),
                    Token::DATA(Data::FLOAT(v)) => (DataType::FLOAT, Value::FLOAT(v)),
                    Token::DATA(Data::STRING(v)) => (DataType::STRING, Value::STRING(v)),
                    Token::DATA(Data::BOOL(v)) => (DataType::BOOL, Value::BOOL(v)),
                    _ => panic!(
                        "Idk, I'm just not bothered for error handling right now. Find this message in the code and figure it out yourself"
                    ),
                }
            }
            _ => unreachable!(),
        };

        if data_type_before.is_some() {
            if data_type_before.clone().unwrap() != data_type {
                return Err(format!(
                    "Data type mismatch: expected {:?}, got {:?}",
                    data_type_before.unwrap(),
                    data_type
                ));
            }
        }

        self.advance();
        match self.expect(vec![Token::SEMICOLON]) {
            Ok(_) => Ok(VariableBinding {
                name: Name { name: name },
                value: value,
                data_type: data_type,
            }),
            Err(e) => return Err("Incorect variable declaration: missing semicolon".into()),
        }
    }

    pub fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, String> {
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

        //Identified colon, which is required by a variable declaration

        match self.advance() {
            Some(Token::COLON) => {}
            _ => return Err("Missing Data Type".into()),
        }

        let mut data_type: DataType;

        match self.advance() {
            Some(Token::DATATYPE(dt)) => data_type = dt.clone(),
            _ => return Err("Missing Data Type".into()),
        }

        self.advance();
        match self.expect(vec![Token::SEMICOLON]) {
            Ok(_) => Ok(VariableDeclaration {
                name: Name { name: name },
                data_type: data_type,
            }),
            Err(e) => return Err("Incorect variable declaration: missing semicolon".into()),
        }
    }

    pub fn parse_variable_assignment(&mut self) -> Result<VariableAssignment, String> {
        let name = match self.current() {
            Some(Token::IDENT(n)) => n,
            None => return Err("Missing Variable Name".into()),
            _ => {
                return Err(format!(
                    "Unexpected Variable Name, got: {:?}",
                    self.current().unwrap().clone()
                ));
            }
        };
        let name: String = name.iter().collect();

        match self.advance() {
            Some(Token::ASSIGN) => {}
            _ => return Err("Missing Variable Declaration".into()),
        }

        let value = match self.advance() {
            Some(v) => match v {
                Token::DATA(v) => v.clone(),
                _ => return Err("Unexpected Variable Value".into()),
            },
            None => {
                return Err("Missing Variable Value".into());
            }
        };

        self.advance().unwrap().clone();
        match self.expect(vec![Token::SEMICOLON]) {
            Ok(_) => Ok(VariableAssignment {
                name: Name { name: name },
                value: data_to_value(value.clone()),
            }),
            Err(e) => return Err("Incorect variable declaration: missing semicolon".into()),
        }
    }

    pub fn parse_variable_modification(&mut self) -> Result<VariableModification, String> {
        let name = match self.current() {
            Some(Token::IDENT(n)) => n,
            None => return Err("Missing Variable Name".into()),
            _ => {
                return Err(format!(
                    "Unexpected Variable Name, got: {:?}",
                    self.current().unwrap().clone()
                ));
            }
        };
        let name: String = name.iter().collect();

        let op = match self.advance() {
            Some(Token::PLUS) => Operator::PLUS,
            Some(Token::MINUS) => Operator::MINUS,
            Some(Token::ASTERISK) => Operator::MUL,
            Some(Token::FSLASH) => Operator::DIV,
            _ => return Err("Missing Variable Modification Operator".into()),
        };

        match self.advance() {
            Some(Token::ASSIGN) => {}
            _ => return Err("Missing Variable Declaration".into()),
        }

        let value = match self.advance() {
            Some(v) => match v {
                Token::DATA(v) => v.clone(),
                _ => return Err("Unexpected Variable Value".into()),
            },
            None => {
                return Err("Missing Variable Value".into());
            }
        };

        self.advance().unwrap().clone();
        match self.expect(vec![Token::SEMICOLON]) {
            Ok(_) => Ok(VariableModification {
                name: Name { name },
                value: data_to_value(value),
                op: op,
            }),
            Err(e) => return Err("Incorect variable declaration: missing semicolon".into()),
        }
    }
}

pub fn data_to_value(data: Data) -> Value {
    match data {
        Data::INT(i) => Value::INT(i),
        Data::FLOAT(f) => Value::FLOAT(f),
        Data::BOOL(b) => Value::BOOL(b),
        Data::STRING(s) => Value::STRING(s),
    }
}
