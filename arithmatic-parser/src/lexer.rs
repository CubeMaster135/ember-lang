#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    NUM(f64),
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    EOO,
}

pub struct Lexer {
    input: String,
    position: usize,
    ch: char,
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            ch: ' ',
        }
    }

    pub fn skip_whitespace(&mut self) {
        let ch = self.ch;
        if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
        }
    }

    pub fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.position).unwrap();
        }
        self.position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let ch = self.ch;
        self.read_char();
        match ch {
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '*' => Token::MUL,
            '/' => Token::DIV,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '\0' => Token::EOO,
            _ => Token::NUM(ch.to_digit(10).unwrap() as f64),
        }
    }
}
