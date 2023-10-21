use std::collections::HashMap;
use std::process::exit;
use std::rc::Rc;

use crate::parse::{Expr, Statement};
use crate::token::Operator;
use crate::types::{Primitive, LogicalAnd, LogicalOr};

struct Context {
    pub vars: HashMap<String, Primitive>,
}

impl Context {
    fn new() -> Self {
        Context {
            vars: HashMap::new(),
        }
    }
}


pub struct Interpreter {
    global_context: Context,

    // 関数の呼び出し時にスタックに積む
    //TODO: 関数実装
    stack: Vec<Context>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            global_context: Context::new(),
            stack: Vec::new(),
        }
    }

    // TODO: こっちをrunにする
    fn run_block(&mut self, statements: Statement) {
        let Statement::Block(statements) = statements else {
            panic!("invalid type")
        };

        self.run(&statements);
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
                    let code = self.eval(expr);
                    exit(code.into());
                }
                Statement::Block(statements) => self.run(statements),

                Statement::If { condition, block, else_block } => {
                    let condition = self.eval(condition);
                    let Primitive::Boolean(condition) = condition else {
                        panic!("invalid type")
                    };

                    if condition {
                        self.run_block(*block.to_owned());
                    } else if let Some(else_block) = else_block {
                        self.run_block(*else_block.to_owned());
                    }
                }
                
            }
        }
    }

    /// 式を評価する
    pub fn eval(&mut self, expr: &Expr) -> Primitive {
        match expr {
            Expr::Identifier(name) => self.eval_identifier(name),
            Expr::Number(n) => Primitive::Number(*n),
            Expr::PrefixExpr { operator, right } => self.eval_prefix_expr(operator, right),
            Expr::InfixExpr {
                left,
                operator,
                right,
            } => self.eval_infix_expr(left, operator, right),
            #[allow(unused_variables)]
            Expr::PostfixExpr { left, operator } => {
                // let left = eval(left);
                // match operator {
                //     _ => panic!("invalid operator"),
                // }
                unimplemented!("postfix operator is not implemented")
            },
            Expr::String(s) => Primitive::String(s.value.clone()),
        }
    }

    fn eval_identifier(&mut self, name: &str) -> Primitive {
        let value = self.global_context.vars.get(name).unwrap_or(&Primitive::Number(0.0));
        match value {
            Primitive::Number(n) => Primitive::Number(*n),
            Primitive::Boolean(b) => Primitive::Boolean(*b),
            Primitive::String(s) => Primitive::String(s.clone()),
            _ => Primitive::Number(0.0)
        }
    }

    fn eval_prefix_expr(&mut self, operator: &Operator, right: &Expr) -> Primitive {
        let right = self.eval(right);
        if let Primitive::Number(right) = right {
            match operator {
                Operator::Plus => Primitive::Number(right),
                Operator::Minus => Primitive::Number(-right),
                Operator::Not => Primitive::Boolean(right == 0.0),
                _ => panic!("invalid operator"),
            }
        } else {
            panic!("invalid operand")
        }
    }

    fn eval_infix_expr(&mut self, left: &Expr, operator: &Operator, right: &Expr) -> Primitive {
        let l_val = &self.eval(left);
        let r_val = &self.eval(right);
        match operator {
            Operator::Plus => l_val + r_val,
            Operator::Minus => l_val - r_val,
            Operator::Mul => l_val * r_val,
            Operator::Div => l_val / r_val,
            Operator::Mod => l_val % r_val,
            Operator::Equal => (l_val == r_val).into(),
            Operator::ObjectEqual => {
                if let Primitive::String(l) = l_val {
                    if let Primitive::String(r) = r_val {
                        Rc::ptr_eq(l, r).into()
                    } else {
                        panic!("invalid type")
                    }
                } else {
                    panic!("invalid type")
                }
            }
            Operator::NotEqual => (l_val != r_val).into(),
            Operator::GreaterThan => (l_val > r_val).into(),
            Operator::GreaterThanEqual => (l_val >= r_val).into(),
            Operator::LessThan => (l_val < r_val).into(),
            Operator::LessThanEqual => (l_val <= r_val).into(),
            Operator::LogicalAnd => l_val.logicaland(&r_val),
            Operator::LogicalOr => l_val.logicalor(&r_val),
            Operator::BitAnd => l_val & r_val,
            Operator::BitOr => l_val| r_val,
            Operator::Assign => {
                self.assign(left, r_val);
                r_val.clone()
            }
            Operator::AddAssign => {
                self.assign(left, &(l_val + r_val));
                l_val + r_val
            },
            Operator::SubAssign => {
                self.assign(left, &(l_val - r_val));
                l_val - r_val
            },
            Operator::MulAssign => {
                self.assign(left, &(l_val * r_val));
                l_val * r_val
            },
            Operator::DivAssign => {
                self.assign(left, &(l_val / r_val));
                l_val / r_val
            },
            Operator::ModAssign => {
                self.assign(left, &(l_val % r_val));
                l_val % r_val
            },
            _ => panic!("invalid operator"),
        }
    }

    fn assign(&mut self, left: &Expr, value: &Primitive) {
        if let Expr::Identifier(name) = left {
            self.global_context.vars.insert(name.clone(), value.clone());
        } else {
            println!("{:?}", left);
            panic!("invalid left hand side of assignment")
        }
    }
}
