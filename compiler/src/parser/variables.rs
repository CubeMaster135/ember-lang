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
        let name = self.parse_variable_name()?;

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
        let name = self.parse_variable_name()?;

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
        let name = self.parse_variable_name()?;

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
        let name = self.parse_variable_name()?;

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

    pub fn parse_variable_name(&mut self) -> Result<String, String> {
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
        Ok(name.iter().collect())
    }

    pub fn parse_datatype(&mut self) -> Result<DataType, String> {
        let mut data_type_before: Option<DataType> = match self.advance().unwrap().clone() {
            Token::DATATYPE(dt) => Some(dt),
            _ => {
                return Err(format!(
                    "Unexpected Data Type, got: {:?}",
                    self.current().unwrap().clone()
                ));
            }
        };
        println!("{:?}", data_type_before);
        data_type_before.ok_or("Missing Data Type".into())
    }

    pub fn parse_operation(&mut self) -> Result<Operator, String> {
        return match self.advance() {
            Some(Token::PLUS) => Ok(Operator::PLUS),
            Some(Token::MINUS) => Ok(Operator::MINUS),
            Some(Token::ASTERISK) => Ok(Operator::MUL),
            Some(Token::FSLASH) => Ok(Operator::DIV),
            _ => return Err("Missing Variable Modification Operator".into()),
        };
    }

    pub fn parse_value(&mut self) -> Result<Data, String> {
        let value = match self.advance() {
            Some(v) => match v {
                Token::DATA(v) => v.clone(),
                _ => return Err("Unexpected Variable Value".into()),
            },
            None => {
                return Err("Missing Variable Value".into());
            }
        };
        Ok(value)
    }

    pub fn parse_semicolon(&mut self) -> Result<(), String> {
        match self.expect(vec![Token::SEMICOLON]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn parse_variable_manipulation(&mut self) -> Result<VariableManipulation, String> {
        let is_let_keyword = self.expect(vec![Token::LET]) == Ok(Token::LET);
        if is_let_keyword {
            self.advance();
        }
        let name = self.parse_variable_name()?;
        let is_type_specified = self.expect(vec![Token::COLON]) == Ok(Token::COLON);
        let dt: Option<DataType> = if is_type_specified {
            self.advance();
            let data_type = self.parse_datatype()?;
            Some(data_type)
        } else {
            None
        };

        let mut is_modification = self.expect(vec![Token::ASSIGN]) == Ok(Token::ASSIGN);
        let is_modification = !is_modification;
        println!("{} {:?}", is_modification, self.current());
        let mut op: Option<Operator> = None;
        if is_modification {
            op = Some(self.parse_operation()?);
        }
        self.advance();
        let value = self.parse_value()?;
        self.advance().unwrap().clone();
        match self.expect(vec![Token::SEMICOLON]) {
            Ok(_) => {
                if is_modification {
                    return Ok(VariableManipulation::Modification(VariableModification {
                        name: Name { name },
                        op: op.unwrap(),
                        value: data_to_value(value),
                    }));
                }
                if is_type_specified {
                    return Ok(VariableManipulation::Binding(VariableBinding {
                        name: Name { name },
                        value: data_to_value(value),
                        data_type: dt.unwrap(),
                    }));
                }
                if is_let_keyword {
                    return Ok(VariableManipulation::Declaration(VariableDeclaration {
                        name: Name { name },
                        data_type: dt.unwrap(),
                    }));
                }
                return Ok(VariableManipulation::Assignment(VariableAssignment {
                    name: Name { name },
                    value: data_to_value(value),
                }));
            }
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
