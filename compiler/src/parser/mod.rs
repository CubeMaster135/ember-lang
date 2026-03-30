pub mod parser;
use crate::lexer::token::DataType;

#[derive(Clone, Debug)]
enum Operator {
    ADD,
    SUB,
    MUL,
    DIV,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Value(f64),
    Variable(String),
    Binary {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    // This represents: name(args) = body
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
}
impl Expr {
    fn new_binary(operator: Operator, left: Box<Expr>, right: Box<Expr>) -> Self {
        Self::Binary {
            op: operator,
            left,
            right,
        }
    }
}

#[derive(Clone, Debug)]
enum Value {
    STRING(String),
    INT(i64),
    FLOAT(f64),
    BOOL(bool),
}

#[derive(Clone, Debug)]
struct Name {
    name: String,
}
impl Name {
    fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    name: Name,
    value: Value,
    data_type: DataType,
}
impl Variable {
    fn new(name: Name, value: Value, data_type: DataType) -> Self {
        Self {
            name,
            value,
            data_type,
        }
    }
}
