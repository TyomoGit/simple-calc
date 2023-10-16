use std::collections::HashMap;

use crate::parse::{Expr, Statement};
use crate::token::Operator;


pub struct Interpreter {
    global_vars: HashMap<String, f64>,

    // 関数の呼び出し時にスタックに積む
    //TODO: 関数実装
    stack: Vec<Context>,
}

struct Context {
    vars: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {

        let mut map = HashMap::new();
        map.insert(String::from("n"), 100f64);

        Interpreter {
            global_vars: map,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self, statements: &[Statement]) {
        for statement in statements {
            match statement {
                Statement::Expr(expr) => {
                    self.eval(expr);
                }
                Statement::Print(expr) => {
                    println!("{}", self.eval(expr));
                }
                Statement::Return(expr) => {
                    println!("{}", self.eval(expr));
                }
            }
        }
    }

    /// 式を評価する
    pub fn eval(&mut self, expr: &Expr) -> f64 {
        match expr {
            Expr::Identifier(name) => {
                if let Some(value) = self.global_vars.get(name) {
                    *value
                } else {
                    0.0
                }
            }
            Expr::Number(n) => *n,
            Expr::PrefixExpr { operator, right } => {
                let right = self.eval(right);
                match operator {
                    Operator::Plus => right,
                    Operator::Minus => -right,
                    Operator::Not => (right == 0.0) as i32 as f64,
                    _ => panic!("invalid operator"),
                }
            }
            Expr::InfixExpr {
                left,
                operator,
                right,
            } => {
                let l_val = self.eval(left);
                let r_val = self.eval(right);
                match operator {
                    Operator::Plus => l_val + r_val,
                    Operator::Minus => l_val - r_val,
                    Operator::Mul => l_val * r_val,
                    Operator::Div => l_val / r_val,
                    Operator::Mod => l_val % r_val,
                    Operator::Equal => (l_val == r_val) as i32 as f64,
                    Operator::NotEqual => (l_val != r_val) as i32 as f64,
                    Operator::GreaterThan => (l_val > r_val) as i32 as f64,
                    Operator::GreaterThanEqual => (l_val >= r_val) as i32 as f64,
                    Operator::LessThan => (l_val < r_val) as i32 as f64,
                    Operator::LessThanEqual => (l_val <= r_val) as i32 as f64,
                    Operator::LogicalAnd => (l_val != 0.0 && r_val != 0.0) as i32 as f64,
                    Operator::LogicalOr => (l_val != 0.0 || r_val != 0.0) as i32 as f64,
                    Operator::BitAnd => (l_val as i32 & r_val as i32) as f64,
                    Operator::BitOr => (l_val as i32 | r_val as i32) as f64,
                    Operator::Assign => {
                        self.assign(left, r_val);
                        r_val
                    }
                    Operator::AddAssign => {
                        self.assign(left, l_val + r_val);
                        l_val + r_val
                    },
                    Operator::SubAssign => {
                        self.assign(left, l_val - r_val);
                        l_val - r_val
                    },
                    Operator::MulAssign => {
                        self.assign(left, l_val * r_val);
                        l_val * r_val
                    },
                    Operator::DivAssign => {
                        self.assign(left, l_val / r_val);
                        l_val / r_val
                    },
                    Operator::ModAssign => {
                        self.assign(left, l_val % r_val);
                        l_val % r_val
                    },
                    _ => panic!("invalid operator"),
                }
            }
            #[allow(unused_variables)]
            Expr::PostfixExpr { left, operator } => {
                // let left = eval(left);
                // match operator {
                //     _ => panic!("invalid operator"),
                // }
                unimplemented!("postfix operator is not implemented")
            },
        }
    }

    fn assign(&mut self, left: &Expr, value: f64) {
        if let Expr::Identifier(name) = left {
            self.global_vars.insert(name.clone(), value);
        } else {
            panic!("invalid left hand side of assignment")
        }
    }
}
