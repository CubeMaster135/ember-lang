pub mod parser;
pub mod variables;
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
pub struct VariableBinding {
    name: Name,
    value: Value,
    data_type: DataType,
}
impl VariableBinding {
    fn new(name: Name, value: Value, data_type: DataType) -> Self {
        Self {
            name,
            value,
            data_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    name: Name,
    data_type: DataType,
}
impl VariableDeclaration {
    fn new(name: Name, data_type: DataType) -> Self {
        Self { name, data_type }
    }
}

#[derive(Clone, Debug)]
pub struct VariableDefinition {
    name: Name,
    value: Value,
}
impl VariableDefinition {
    fn new(name: Name, value: Value) -> Self {
        Self { name, value }
    }
}

#[derive(Clone, Debug)]
pub struct VariableModification {
    name: Name,
    value: Value,
    op: Operator,
}
impl VariableModification {
    fn new(name: Name, value: Value, op: Operator) -> Self {
        Self { name, value, op }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    name: Name,
    args: Vec<Expr>,
}
