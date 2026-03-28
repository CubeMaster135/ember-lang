use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Operator {
    PLUS,
    MINUS,
    MUL,
    DIV,
}

#[derive(Debug, Clone)]
pub struct Constant {
    value: f64,
}

impl Constant {
    pub fn value(&self) -> f64 {
        self.value
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Operator {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Constant(Constant),
}

#[derive(Debug)]
pub struct Parser {
    input: Vec<Token>,
    position: usize,
    curr: Option<Token>,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            position: 0,
            curr: None,
        }
    }

    fn read_next(&mut self) -> Option<Token> {
        self.input.get(self.position).copied().map(|t| {
            self.position += 1;
            self.curr = Some(t);
            t
        })
    }

    fn expect(&mut self, token: Token) -> Option<Token> {
        match self.curr {
            Some(t) if t == token => {
                self.read_next(); // consume it
                Some(t)
            }
            _ => None,
        }
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        match self.curr {
            Some(Token::NUM(value)) => {
                self.read_next();
                Ok(Expression::Constant(Constant { value }))
            }
            Some(Token::LPAREN) => {
                self.read_next(); // consume '('
                let result = self.parse_expr();
                let rparen = self.expect(Token::RPAREN);
                match rparen {
                    Some(_) => Ok(result.unwrap()),
                    None => Err(format!("Missing closing parenthesis")),
                }
            }
            _ => Err(format!("Unexpected token: {:?}", self.curr.unwrap())),
        }
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let left = self.parse_factor();
        match left {
            Ok(left) => {
                let mut result = left.clone();
                while self.curr == Some(Token::MUL) || self.curr == Some(Token::DIV) {
                    let op = match self.curr.unwrap() {
                        Token::MUL => Operator::MUL,
                        Token::DIV => Operator::DIV,
                        _ => unreachable!(),
                    };
                    self.read_next();
                    let right = self.parse_factor();
                    match right {
                        Ok(right) => {
                            result = Expression::Operator {
                                operator: op,
                                left: Box::new(result),
                                right: Box::new(right),
                            };
                        }
                        Err(e) => return Err(e),
                    }
                }
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    fn parse_expr(&mut self) -> Result<Expression, String> {
        let left = self.parse_term();
        match left {
            Ok(left) => {
                let mut result = left.clone();
                while self.curr == Some(Token::PLUS) || self.curr == Some(Token::MINUS) {
                    let op = match self.curr.unwrap() {
                        Token::PLUS => Operator::PLUS,
                        Token::MINUS => Operator::MINUS,
                        _ => unreachable!(),
                    };
                    self.read_next();
                    let right = self.parse_term();
                    match right {
                        Ok(right) => {
                            result = Expression::Operator {
                                operator: op,
                                left: Box::new(result),
                                right: Box::new(right),
                            };
                        }
                        Err(e) => return Err(e),
                    }
                }
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.read_next(); // prime curr
        self.parse_expr()
    }
}
