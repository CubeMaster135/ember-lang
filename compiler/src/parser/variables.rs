use std::arch::x86_64::_MM_ROUND_TOWARD_ZERO;
use std::ops::BitOrAssign;

use crate::lexer::token::*;
use crate::parser::parser::*;
use crate::parser::*;

impl Parser {
    pub fn parse_variable_name(&mut self) -> Option<String> {
        let name = match self.current().clone() {
            Some(Token::IDENT(n)) => {
                let name: String = n.iter().collect(); // collect into owned String immediately
                self.advance(); // immutable borrow is now gone
                name
            }
            None => return None,
            _ => return None,
        };
        Some(name)
    }

    pub fn parse_datatype(&mut self) -> Option<DataType> {
        match self.current().unwrap().clone() {
            Token::DATATYPE(dt) => {
                self.advance();
                Some(dt)
            }
            _ => {
                return None;
            }
        }
    }

    pub fn parse_operation(&mut self, op: Option<Token>) -> Option<Operator> {
        match op {
            Some(Token::OP(OperatorToken::PLUSEQUALS)) => Some(Operator::PLUS),
            Some(Token::OP(OperatorToken::MINUSEQUALS)) => Some(Operator::MINUS),
            Some(Token::OP(OperatorToken::TIMESEQUALS)) => Some(Operator::MUL),
            Some(Token::OP(OperatorToken::DIVEQUALS)) => Some(Operator::DIV),
            Some(Token::ASSIGN) => Some(Operator::EQUALS),
            _ => unreachable!(),
        }
    }

    pub fn parse_value(&mut self) -> Option<(Operator, Data)> {
        let modification = self.expect(vec![
            Token::OP(OperatorToken::PLUSEQUALS),
            Token::OP(OperatorToken::MINUSEQUALS),
            Token::OP(OperatorToken::TIMESEQUALS),
            Token::OP(OperatorToken::DIVEQUALS),
            Token::ASSIGN,
        ]);

        if modification.is_some() {
            let op = self.parse_operation(modification);

            let value = match self.current() {
                Some(v) => match v {
                    Token::DATA(v) => {
                        let value = v.clone();
                        self.advance();
                        Some(value)
                    }
                    _ => None,
                },
                None => None,
            };

            Some((op.unwrap(), value.unwrap()))
        } else {
            None
        }
    }

    pub fn parse_type_specified(&mut self) -> Option<DataType> {
        if self.expect(vec![Token::COLON]).is_some() {
            let data_type = self.parse_datatype()?;
            Some(data_type)
        } else {
            None
        }
    }

    pub fn print_current(&self) {
        println!("Current Token: {:?}", self.current());
    }

    pub fn parse_variable_manipulation(&mut self) -> Result<VariableManipulation, String> {
        // Checks if let keyword is present
        let let_keyword: Option<Token> = self.expect(vec![Token::LET]);

        // Gets the variable name (required)
        let name = self
            .parse_variable_name()
            .ok_or(String::from("Missing Variable Name"))?;

        // Checks if a type is specified
        let mut data_type = self.parse_type_specified();

        // Checks if a modification operator is present
        let mut operation = None;
        let mut value = None;
        let temp = self.parse_value();
        if temp.is_some() {
            let (op, val) = temp.unwrap();
            operation = Some(op);
            value = Some(val);
        }

        match self.expect(vec![Token::SEMICOLON]) {
            Some(_) => {
                if operation.is_some() && operation.clone().unwrap() != Operator::EQUALS {
                    return Ok(VariableManipulation::Modification(VariableModification {
                        name: Name { name },
                        op: operation.unwrap(),
                        value: data_to_value(value.unwrap()),
                    }));
                }
                if let_keyword.is_none() {
                    return Ok(VariableManipulation::Assignment(VariableAssignment {
                        name: Name { name },
                        value: data_to_value(value.unwrap()),
                    }));
                }
                if data_type.is_some() && value.is_none() {
                    return Ok(VariableManipulation::Declaration(VariableDeclaration {
                        name: Name { name },
                        data_type: data_type.unwrap(),
                    }));
                }

                if data_type.is_some() && value.is_some() {
                    let dt1 = data_type.clone().unwrap();
                    let dt2 = value.clone().unwrap().data_type();

                    if dt1 != dt2 {
                        if dt1 == DataType::FLOAT && dt2 == DataType::INT {
                            if let Data::INT(i) = value.clone().unwrap() {
                                value = Some(Data::FLOAT(i as f64));
                            }
                        } else if dt1 == DataType::INT && dt2 == DataType::FLOAT {
                            if let Data::FLOAT(f) = value.clone().unwrap() {
                                value = Some(Data::INT(f as i64));
                            }
                        } else {
                            return Err("Incorrect variable declaration: data type mismatch".into());
                        }
                    }
                }

                if data_type.is_none() {
                    data_type = Some(value.clone().unwrap().data_type());
                }
                return Ok(VariableManipulation::Binding(VariableBinding {
                    name: Name { name },
                    value: data_to_value(value.unwrap()),
                    data_type: data_type.unwrap(),
                }));
            }
            None => return Err("Incorect variable declaration: missing semicolon".into()),
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
