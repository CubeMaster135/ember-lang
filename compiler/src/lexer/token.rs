#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    INT,
    FLOAT,
    BOOL,
    STRING,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(Vec<char>),
    INT(Vec<char>),
    ASSIGN,
    PLUS(char),
    COMMA(char),
    COLON,
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
    DATATYPE(DataType),
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
        "int" => Ok(Token::DATATYPE(DataType::INT)),
        "float" => Ok(Token::DATATYPE(DataType::FLOAT)),
        "bool" => Ok(Token::DATATYPE(DataType::BOOL)),
        "string" => Ok(Token::DATATYPE(DataType::STRING)),
        _ => Err(String::from("Not a keyword")),
    }
}
