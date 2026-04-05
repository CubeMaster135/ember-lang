use crate::lexer::token::*;
use crate::parser::parser::*;
use crate::parser::*;

impl Parser {
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
            Some(Token::OP(OperatorToken::PLUS)) => Ok(Operator::PLUS),
            Some(Token::OP(OperatorToken::MINUS)) => Ok(Operator::MINUS),
            Some(Token::OP(OperatorToken::TIMES)) => Ok(Operator::MUL),
            Some(Token::OP(OperatorToken::DIVIDE)) => Ok(Operator::DIV),
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

    pub fn print_current(&self) {
        println!("Current Token: {:?}", self.current());
    }

    pub fn parse_variable_manipulation(&mut self) -> Result<VariableManipulation, String> {
        // Checks if let keyword is present
        let is_let_keyword = self.expect(vec![Token::LET]) == Ok(Token::LET);
        if is_let_keyword {
            self.advance();
        }

        // Gets the variable name
        let name = self.parse_variable_name()?;
        self.advance();

        let is_type_specified = self.expect(vec![Token::COLON]) == Ok(Token::COLON);
        let mut dt: Option<DataType> = None;
        if is_type_specified {
            let data_type = self.parse_datatype()?;
            dt = Some(data_type);
        }

        self.advance();
        self.print_current();
        let mut is_modification = self
            .expect(vec![
                Token::OP(OperatorToken::PLUS),
                Token::OP(OperatorToken::MINUS),
                Token::OP(OperatorToken::TIMES),
                Token::OP(OperatorToken::DIVIDE),
            ])
            .is_ok();
        println!("{} {:?}", is_modification, self.current());
        let mut op: Option<Operator> = None;
        if is_modification {
            op = Some(self.parse_operation()?);
        }
        let value = self.parse_value()?;

        self.advance();
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
