use colored::Colorize;

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
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.consume();
        }
    }

    pub fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.position).unwrap_or('\0');
    }

    pub fn consume(&mut self) -> char {
        let ch = self.ch;
        self.position += 1;
        self.read_char();
        ch
    }

    pub fn read_number(&mut self) -> Result<String, String> {
        let mut number = String::new();
        number.push(self.ch); // capture the first digit already in self.ch
        self.consume(); // advance WITHOUT consuming (just move forward)
        while is_digit(self.ch) || self.ch == '.' {
            number.push(self.ch);
            self.consume();
        }
        Ok(number)
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.read_char();
        self.skip_whitespace();
        let ch = self.ch;
        match ch {
            '+' => {
                self.consume();
                Ok(Token::PLUS)
            }
            '-' => {
                self.consume();
                Ok(Token::MINUS)
            }
            '*' => {
                self.consume();
                Ok(Token::MUL)
            }
            '/' => {
                self.consume();
                Ok(Token::DIV)
            }
            '(' => {
                self.consume();
                Ok(Token::LPAREN)
            }
            ')' => {
                self.consume();
                Ok(Token::RPAREN)
            }
            '\0' => Ok(Token::EOO),
            _ if is_digit(ch) => {
                let str = self.read_number()?;
                Ok(Token::NUM(str.trim().parse::<f64>().unwrap()))
            }
            _ => Err(format!("Character {} not recognised", ch)),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut result = Vec::new();
        loop {
            let token = self.next_token();
            match token {
                Ok(token) => {
                    result.push(token);
                    if token == Token::EOO {
                        break;
                    }
                }
                Err(e) => {
                    println!("{}", format!("LEXER ERROR: {}", e).red().bold());
                }
            }
        }
        result
    }
}
