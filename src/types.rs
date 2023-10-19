use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg, BitAnd, BitOr};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Number(f64),
    Boolean(bool),
    String(Rc<String>),
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Number(n) => write!(f, "{}", n),
            Primitive::Boolean(b) => write!(f, "{}", b),
            Primitive::String(s) => write!(f, "{}", s),
        }
    }
}

impl Add for &Primitive {
    type Output = Primitive;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Number(l), Primitive::Number(r)) => Primitive::Number(l + r),
            (Primitive::String(l), Primitive::String(r)) => Primitive::String((l.to_string() + r).into()),
            _ => panic!("invalid type"),
        }
    }
}

impl Sub for &Primitive {
    type Output = Primitive;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Number(l), Primitive::Number(r)) => Primitive::Number(l - r),
            _ => panic!("invalid type"),
        }
    }
}

impl Mul for &Primitive {
    type Output = Primitive;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Number(l), Primitive::Number(r)) => Primitive::Number(l * r),
            _ => panic!("invalid type"),
        }
    }
}

impl Div for &Primitive {
    type Output = Primitive;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Number(l), Primitive::Number(r)) => Primitive::Number(l / r),
            _ => panic!("invalid type"),
        }
    }
}

impl Rem for &Primitive {
    type Output = Primitive;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Number(l), Primitive::Number(r)) => Primitive::Number(l % r),
            _ => panic!("invalid type"),
        }
    }
}


impl Neg for &Primitive {
    type Output = Primitive;
    fn neg(self) -> Self::Output {
        match self {
            Primitive::Number(n) => Primitive::Number(-n),
            _ => panic!("invalid type"),
        }
    }
}

impl BitAnd for &Primitive {
    type Output = Primitive;
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Number(l), Primitive::Number(r)) => Primitive::Number((*l as i32 & *r as i32) as f64),
            _ => panic!("invalid type"),
        }
    }
}

impl BitOr for &Primitive{
    type Output = Primitive;
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Number(l), Primitive::Number(r)) => Primitive::Number((*l as i32 | *r as i32) as f64),
            _ => panic!("invalid type"),
        }
    }
}

// impl Into<bool> for Primitive {
//     fn into(self) -> bool {
//         match self {
//             Primitive::Number(n) => n != 0.0,
//             Primitive::Boolean(b) => b,
//             _ => panic!("invalid type"),
//         }
//     }
// }

impl From<bool> for Primitive {
    fn from(value: bool) -> Self {
        Primitive::Boolean(value)
    }
}

pub trait LogicalAnd {
    type Output;
    fn logicaland(&self, rhs: &Self) -> Self::Output;
}

pub trait LogicalOr {
    type Output;
    fn logicalor(&self, rhs: &Self) -> Self::Output;
}

impl LogicalAnd for &Primitive{
    type Output = Primitive;
    fn logicaland(&self, rhs: &Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Boolean(l), Primitive::Boolean(r)) => Primitive::Boolean(*l && *r),
            _ => panic!("invalid type"),
        }
    }
}

impl LogicalOr for &Primitive {
    type Output = Primitive;
    fn logicalor(&self, rhs: &Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Boolean(l), Primitive::Boolean(r)) => Primitive::Boolean(*l || *r),
            _ => panic!("invalid type"),
        }
    }
}

impl From<Primitive> for i32 {
    fn from(val: Primitive) -> Self {
        match val {
            Primitive::Number(n) => n as i32,
            Primitive::Boolean(b) => b as i32,
            _ => panic!("invalid type"),
        }
    }
}