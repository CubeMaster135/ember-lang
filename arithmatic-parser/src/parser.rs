use crate::lexer::Token;

#[derive(Debug)]
pub enum Operator {
    PLUS,
    MINUS,
    MUL,
    DIV,
}

#[derive(Debug)]
pub struct Constant {
    value: f64,
}
#[derive(Debug)]
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
    tree: Option<Expression>,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            position: 0,
            curr: None,
            tree: None,
        }
    }

    pub fn tree(&self) -> Option<&Expression> {
        self.tree.as_ref()
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

    fn parse_factor(&mut self) -> Option<Expression> {
        match self.curr {
            Some(Token::NUM(value)) => {
                self.read_next();
                Some(Expression::Constant(Constant { value }))
            }
            Some(Token::LPAREN) => {
                self.read_next(); // consume '('
                let result = self.parse_expr();
                self.expect(Token::RPAREN)?;
                result
            }
            _ => None,
        }
    }

    fn parse_term(&mut self) -> Option<Expression> {
        let mut left = self.parse_factor()?;
        while self.curr == Some(Token::MUL) || self.curr == Some(Token::DIV) {
            let op = match self.curr.unwrap() {
                Token::MUL => Operator::MUL,
                Token::DIV => Operator::DIV,
                _ => unreachable!(),
            };
            self.read_next();
            let right = self.parse_factor()?;
            left = Expression::Operator {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Some(left)
    }

    fn parse_expr(&mut self) -> Option<Expression> {
        let mut left = self.parse_term()?;
        while self.curr == Some(Token::PLUS) || self.curr == Some(Token::MINUS) {
            let op = match self.curr.unwrap() {
                Token::PLUS => Operator::PLUS,
                Token::MINUS => Operator::MINUS,
                _ => unreachable!(),
            };
            self.read_next();
            let right = self.parse_term()?;
            left = Expression::Operator {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Some(left)
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.read_next(); // prime curr
        self.tree = self.parse_expr();
        Ok(())
    }
}
