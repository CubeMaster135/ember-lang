pub mod parser;

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
struct Value {
    value: String,
}
impl Value {
    fn new(value: String) -> Self {
        Self { value }
    }
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
}
impl Variable {
    fn new(name: Name, value: Value) -> Self {
        Self { name, value }
    }
    fn get_value(&self) -> &String {
        &self.value.value
    }
    fn set_value(&mut self, value: String) {
        self.value.value = value;
    }
}
