use std::string;

use crate::{
    lexer::token::{Data, DataType, Token},
    parser::*,
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        self.pos += 1;
        self.tokens.get(self.pos)
    }

    pub fn cut(&mut self) {
        self.tokens.remove(self.pos);
    }

    pub fn expect(&mut self, expected: Vec<Token>) -> Option<Token> {
        for token in &expected {
            if *self.current().unwrap() == *token {
                self.advance();
                return Some(token.clone());
            }
        }
        return None;
    }
}
