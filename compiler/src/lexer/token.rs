#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(Vec<char>),
    INT(Vec<char>),
    ASSIGN,
    PLUS(char),
    COMMA(char),
    SEMICOLON,
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    MINUS(char),
    BANG(char),
    ASTERISK(char),
    FSLASH(char),
    BSLASH(char),
    LT(char),
    GT(char),
    QMARK,
    SMARK,
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "fn" => Ok(Token::FUNCTION),
        "let" => Ok(Token::LET),
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        "if" => Ok(Token::IF),
        "else" => Ok(Token::ELSE),
        "return" => Ok(Token::RETURN),
        _ => Err(String::from("Not a keyword")),
    }
}
