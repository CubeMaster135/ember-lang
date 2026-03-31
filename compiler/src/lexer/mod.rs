pub mod token;

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn skip_whitespace(&mut self) {
        let ch = self.ch;
        if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Vec<char> {
        let position = self.position;
        while self.position < self.input.len() && (is_digit(self.ch) || self.ch == '.') {
            self.read_char();
        }
        self.input[position..self.position].to_vec()
    }

    pub fn next_token(&mut self) -> token::Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_letter(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let tok: token::Token;
        self.skip_whitespace();
        match self.ch {
            '=' => tok = token::Token::ASSIGN,
            '+' => tok = token::Token::PLUS(self.ch),
            '-' => tok = token::Token::MINUS(self.ch),
            '!' => tok = token::Token::BANG(self.ch),
            '/' => tok = token::Token::FSLASH(self.ch),
            '\\' => tok = token::Token::BSLASH(self.ch),
            '*' => tok = token::Token::ASTERISK(self.ch),
            '<' => tok = token::Token::LT(self.ch),
            '>' => tok = token::Token::GT(self.ch),
            ':' => tok = token::Token::COLON,
            ';' => tok = token::Token::SEMICOLON,
            '(' => tok = token::Token::LPAREN(self.ch),
            ')' => tok = token::Token::RPAREN(self.ch),
            ',' => tok = token::Token::COMMA(self.ch),
            '{' => tok = token::Token::LBRACE(self.ch),
            '}' => tok = token::Token::RBRACE(self.ch),
            '0' => tok = token::Token::EOF,
            '\'' => tok = token::Token::QMARK, // Single instance of single quote
            '\"' => tok = {
                self.read_char();
                let start = self.position;
                while self.ch != '\"' {
                    self.read_char();
                }
                let chars: Vec<char> = self.input[start..self.position].iter().copied().collect();
                token::Token::DATA(token::Data::STRING(chars.into_iter().collect()))
            },
            ' ' => {
                self.skip_whitespace();
                return self.next_token();
            }
            _ => {
                if is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    match token::get_keyword_token(&ident) {
                        Ok(keywork_token) => {
                            return keywork_token;
                        }
                        Err(_err) => {
                            return token::Token::IDENT(ident);
                        }
                    }
                } else if is_digit(self.ch) {
                    let ident: Vec<char> = self.read_number();
                    if ident.iter().collect::<String>().parse::<i64>().is_ok() {
                        return token::Token::DATA(token::Data::INT(ident.iter().collect::<String>().parse::<i64>().unwrap()));
                    } else if ident.iter().collect::<String>().parse::<f64>().is_ok() {
                        return token::Token::DATA(token::Data::FLOAT(ident.iter().collect::<String>().parse::<f64>().unwrap()));
                    } else {
                        return token::Token::ILLEGAL;
                    }
                } else {
                    return token::Token::ILLEGAL;
                }
            }
        }
        self.read_char();
        tok
    }
}
