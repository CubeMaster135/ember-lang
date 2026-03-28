use crate::parser::{Expression, Operator};

pub struct Solver {
    pub ast: Expression,
}
impl Solver {
    pub fn new(ast: Expression) -> Self {
        Self { ast }
    }

    pub fn solve_expr(&self, expr: &Expression) -> f64 {
        match expr {
            Expression::Constant(c) => c.value(),
            Expression::Operator {
                operator,
                left,
                right,
            } => {
                let left_val = self.solve_expr(left);
                let right_val = self.solve_expr(right);
                match operator {
                    Operator::PLUS => left_val + right_val,
                    Operator::MINUS => left_val - right_val,
                    Operator::MUL => left_val * right_val,
                    Operator::DIV => left_val / right_val,
                }
            }
        }
    }
}
