#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    INT,
    FLOAT,
    BOOL,
    STRING,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Data {
    INT(i64),
    FLOAT(f64),
    BOOL(bool),
    STRING(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum OperatorToken {
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    PLUSEQUALS,
    MINUSEQUALS,
    TIMESEQUALS,
    DIVEQUALS,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(Vec<char>),
    INT(Vec<char>),
    ASSIGN,
    COMMA,
    COLON,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    BANG,
    BSLASH,
    LT,
    GT,
    QMARK,
    SMARK,
    DATATYPE(DataType),
    DATA(Data),
    OP(OperatorToken),
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "fn" => Ok(Token::FUNCTION),
        "let" => Ok(Token::LET),
        "true" => Ok(Token::DATA(Data::BOOL(true))),
        "false" => Ok(Token::DATA(Data::BOOL(false))),
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
