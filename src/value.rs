use crate::{
    number::Number,
    parser::Node
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(Number),
    String(String),
    Boolean(bool),
    Vector(Vec<Value>),
    Function(Vec<Node>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let (Value::Number(num1), Value::Number(num2)) = (self, other) {
            num1.partial_cmp(num2)
        } else {
            None
        }
    }
}

impl Value {
    pub fn cast_to(&self, result_type: String) -> Result<Value, String> {
        match self {
            Value::Number(value) => Ok(Value::Number(value.cast_to(result_type)?)),
            _ => Err("Cannot cast this type".to_string())
        }
    }
}

impl std::ops::Add for Value {
    type Output = Result<Self, String>;
    fn add(self, other: Value) -> Result<Self, String> {
        match (self, other) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number((v1+v2)?)),
            _ => Err("Invalid types for '+'".to_string())
        }
    }
}
impl std::ops::Sub for Value {
    type Output = Result<Self, String>;
    fn sub(self, other: Value) -> Result<Self, String> {
        match (self, other) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number((v1-v2)?)),
            _ => Err("Invalid types for '-'".to_string())
        }
    }
}
impl std::ops::Mul for Value {
    type Output = Result<Self, String>;
    fn mul(self, other: Value) -> Result<Self, String> {
        match (self, other) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number((v1*v2)?)),
            _ => Err("Invalid types for '*'".to_string())
        }
    }
}
impl std::ops::Div for Value {
    type Output = Result<Self, String>;
    fn div(self, other: Value) -> Result<Self, String> {
        match (self, other) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number((v1/v2)?)),
            _ => Err("Invalid types for '/'".to_string())
        }
    }
}
impl std::ops::Rem for Value {
    type Output = Result<Self, String>;
    fn rem(self, other: Value) -> Result<Self, String> {
        match (self, other) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number((v1%v2)?)),
            _ => Err("Invalid types for '%'".to_string())
        }
    }
}
