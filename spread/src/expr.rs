
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Add {
    Val(i32),
    Ref(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Mul {
    Val(i32),
    Ref(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Operation {
    Add(Add),
    Mul(Mul),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Expr {
    Numeric(i32),
    Expression(Vec<Operation>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Numeric(val) => write!(f, "{}", val),
            Expr::Expression(operations) => {
                let mut expr_str = String::from("1");
                for operation in operations {
                    match operation {
                        Operation::Add(Add::Val(val)) => {
                            expr_str = format!("{} + {}", expr_str, val)
                        }
                        Operation::Add(Add::Ref(var)) => {
                            expr_str = format!("{} + {}", expr_str, var)
                        }
                        Operation::Mul(Mul::Val(val)) => {
                            expr_str = format!("{}*({})", val, expr_str)
                        }
                        Operation::Mul(Mul::Ref(var)) => {
                            expr_str = format!("{}*({})", var, expr_str)
                        }
                    }
                }
                write!(f, "{}", expr_str)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Assignment {
    pub(crate) var: char,
    pub(crate) expr: Expr,
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.var, self.expr)
    }
}
