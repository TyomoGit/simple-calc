use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg, BitAnd, BitOr};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(Primitive),
    Reference(Rc<Reference>),
    Array(),
}

pub trait TypeName {
    fn type_name(&self) -> &'static str;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub name: String,
    pub field: HashMap<String, Box<Type>>,
}

impl Reference {
    // fn new_string(content: &str) -> Self {
    //     let mut field = HashMap::new();
    //     field.insert("value", )
    // }

    fn new_array() -> Self {
        let mut field = HashMap::new();
        field.insert("length".to_string(), Box::new(Type::Primitive(Primitive::Number(0.0))));
        Reference {
            name: "Array".to_string(),
            field,
        }
    }
}

impl TypeName for Type {
    fn type_name(&self) -> &'static str {
        match self {
            Type::Primitive(p) => p.type_name(),
            Type::Reference(r) => r.type_name(),
        }
    }
}

impl TypeName for Primitive {
    fn type_name(&self) -> &'static str {
        match self {
            Primitive::Number(_) => "number",
            Primitive::Boolean(_) => "boolean",
        }
    }
}

impl TypeName for Reference {
    fn type_name(&self) -> &'static str {
        self.name.as_str()
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Number(n) => write!(f, "{}", n),
            Primitive::Boolean(b) => write!(f, "{}", b),
        }
    }
}

impl Add for &Primitive {
    type Output = Primitive;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Number(l), Primitive::Number(r)) => Primitive::Number(l + r),
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