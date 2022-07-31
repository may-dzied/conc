#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Number {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
}

macro_rules! fallible_cast {
    ($val:expr, $cast_to:ident, $dest_t:ident) => {
        Ok(Number::$cast_to((*$val)
            .try_into()
            .map_err(|_| format!(
                "Error casting '{}' to '{}'",
                $val,
                stringify!($dest_t)
            ))?)
        )
    }
}

macro_rules! cast_block {
    ($obj:expr, $variant:ident, $result_type:ident) => {
        match $obj {
            Number::I8(value) => fallible_cast!(value, $variant, $result_type),
            Number::I16(value) => fallible_cast!(value, $variant, $result_type),
            Number::I32(value) => fallible_cast!(value, $variant, $result_type),
            Number::I64(value) => fallible_cast!(value, $variant, $result_type),
            Number::I128(value) => fallible_cast!(value, $variant, $result_type),
            Number::U8(value) => fallible_cast!(value, $variant, $result_type),
            Number::U16(value) => fallible_cast!(value, $variant, $result_type),
            Number::U32(value) => fallible_cast!(value, $variant, $result_type),
            Number::U64(value) => fallible_cast!(value, $variant, $result_type),
            Number::U128(value) => fallible_cast!(value, $variant, $result_type),
            Number::F32(value) => Ok(Number::$variant(*value as $result_type)),
            Number::F64(value) => Ok(Number::$variant(*value as $result_type)),
        }
    }
}

macro_rules! float_cast_block {
    ($obj:expr, $variant:ident, $result_type:ident) => {
        match $obj {
            Number::I8(value) => Ok(Number::$variant(*value as $result_type)),
            Number::I16(value) => Ok(Number::$variant(*value as $result_type)),
            Number::I32(value) => Ok(Number::$variant(*value as $result_type)),
            Number::I64(value) => Ok(Number::$variant(*value as $result_type)),
            Number::I128(value) => Ok(Number::$variant(*value as $result_type)),
            Number::U8(value) => Ok(Number::$variant(*value as $result_type)),
            Number::U16(value) => Ok(Number::$variant(*value as $result_type)),
            Number::U32(value) => Ok(Number::$variant(*value as $result_type)),
            Number::U64(value) => Ok(Number::$variant(*value as $result_type)),
            Number::U128(value) => Ok(Number::$variant(*value as $result_type)),
            Number::F32(value) => Ok(Number::$variant(*value as $result_type)),
            Number::F64(value) => Ok(Number::$variant(*value as $result_type)),
        }
    }
}

impl Number {
    pub fn cast_to(&self, result_type: String) -> Result<Number, String> {
        match result_type.as_str() {
            "i8" => cast_block!(self, I8, i8),
            "i16" => cast_block!(self, I16, i16),
            "i32" => cast_block!(self, I32, i32),
            "i64" => cast_block!(self, I64, i64),
            "i128" => cast_block!(self, I128, i128),
            "u8" => cast_block!(self, U8, u8),
            "u16" => cast_block!(self, U16, u16),
            "u32" => cast_block!(self, U32, u32),
            "u64" => cast_block!(self, U64, u64),
            "u128" => cast_block!(self, U128, u128),
            "f32" => float_cast_block!(self, F32, f32),
            _ => Err(format!("Unknown number type {}", result_type))
        }
    }
}

impl std::ops::Add for Number {
    type Output = Result<Number, String>;
    fn add(self, other: Number) -> Result<Number, String> {
        match (self, other) {
            (Number::I8(v1), Number::I8(v2)) => Ok(Number::I8(v1+v2)),
            (Number::I16(v1), Number::I16(v2)) => Ok(Number::I16(v1+v2)),
            (Number::I32(v1), Number::I32(v2)) => Ok(Number::I32(v1+v2)),
            (Number::I64(v1), Number::I64(v2)) => Ok(Number::I64(v1+v2)),
            (Number::I128(v1), Number::I128(v2)) => Ok(Number::I128(v1+v2)),
            (Number::U8(v1), Number::U8(v2)) => Ok(Number::U8(v1+v2)),
            (Number::U16(v1), Number::U16(v2)) => Ok(Number::U16(v1+v2)),
            (Number::U32(v1), Number::U32(v2)) => Ok(Number::U32(v1+v2)),
            (Number::U64(v1), Number::U64(v2)) => Ok(Number::U64(v1+v2)),
            (Number::U128(v1), Number::U128(v2)) => Ok(Number::U128(v1+v2)),
            (Number::F32(v1), Number::F32(v2)) => Ok(Number::F32(v1+v2)),
            (Number::F64(v1), Number::F64(v2)) => Ok(Number::F64(v1+v2)),
            _ => Err("Mismatched number types for '+'".to_string())
        }
    }
}
impl std::ops::Sub for Number {
    type Output = Result<Number, String>;
    fn sub(self, other: Number) -> Result<Number, String> {
        match (self, other) {
            (Number::I8(v1), Number::I8(v2)) => Ok(Number::I8(v1-v2)),
            (Number::I16(v1), Number::I16(v2)) => Ok(Number::I16(v1-v2)),
            (Number::I32(v1), Number::I32(v2)) => Ok(Number::I32(v1-v2)),
            (Number::I64(v1), Number::I64(v2)) => Ok(Number::I64(v1-v2)),
            (Number::I128(v1), Number::I128(v2)) => Ok(Number::I128(v1-v2)),
            (Number::U8(v1), Number::U8(v2)) => Ok(Number::U8(v1-v2)),
            (Number::U16(v1), Number::U16(v2)) => Ok(Number::U16(v1-v2)),
            (Number::U32(v1), Number::U32(v2)) => Ok(Number::U32(v1-v2)),
            (Number::U64(v1), Number::U64(v2)) => Ok(Number::U64(v1-v2)),
            (Number::U128(v1), Number::U128(v2)) => Ok(Number::U128(v1-v2)),
            (Number::F32(v1), Number::F32(v2)) => Ok(Number::F32(v1-v2)),
            (Number::F64(v1), Number::F64(v2)) => Ok(Number::F64(v1-v2)),
            _ => Err("Mismatched number types for '-'".to_string())
        }
    }
}
impl std::ops::Mul for Number {
    type Output = Result<Number, String>;
    fn mul(self, other: Number) -> Result<Number, String> {
        match (self, other) {
            (Number::I8(v1), Number::I8(v2)) => Ok(Number::I8(v1*v2)),
            (Number::I16(v1), Number::I16(v2)) => Ok(Number::I16(v1*v2)),
            (Number::I32(v1), Number::I32(v2)) => Ok(Number::I32(v1*v2)),
            (Number::I64(v1), Number::I64(v2)) => Ok(Number::I64(v1*v2)),
            (Number::I128(v1), Number::I128(v2)) => Ok(Number::I128(v1*v2)),
            (Number::U8(v1), Number::U8(v2)) => Ok(Number::U8(v1*v2)),
            (Number::U16(v1), Number::U16(v2)) => Ok(Number::U16(v1*v2)),
            (Number::U32(v1), Number::U32(v2)) => Ok(Number::U32(v1*v2)),
            (Number::U64(v1), Number::U64(v2)) => Ok(Number::U64(v1*v2)),
            (Number::U128(v1), Number::U128(v2)) => Ok(Number::U128(v1*v2)),
            (Number::F32(v1), Number::F32(v2)) => Ok(Number::F32(v1*v2)),
            (Number::F64(v1), Number::F64(v2)) => Ok(Number::F64(v1*v2)),
            _ => Err("Mismatched number types for '*'".to_string())
        }
    }
}
impl std::ops::Div for Number {
    type Output = Result<Number, String>;
    fn div(self, other: Number) -> Result<Number, String> {
        match (self, other) {
            (Number::I8(v1), Number::I8(v2)) => Ok(Number::I8(v1/v2)),
            (Number::I16(v1), Number::I16(v2)) => Ok(Number::I16(v1/v2)),
            (Number::I32(v1), Number::I32(v2)) => Ok(Number::I32(v1/v2)),
            (Number::I64(v1), Number::I64(v2)) => Ok(Number::I64(v1/v2)),
            (Number::I128(v1), Number::I128(v2)) => Ok(Number::I128(v1/v2)),
            (Number::U8(v1), Number::U8(v2)) => Ok(Number::U8(v1/v2)),
            (Number::U16(v1), Number::U16(v2)) => Ok(Number::U16(v1/v2)),
            (Number::U32(v1), Number::U32(v2)) => Ok(Number::U32(v1/v2)),
            (Number::U64(v1), Number::U64(v2)) => Ok(Number::U64(v1/v2)),
            (Number::U128(v1), Number::U128(v2)) => Ok(Number::U128(v1/v2)),
            (Number::F32(v1), Number::F32(v2)) => Ok(Number::F32(v1/v2)),
            (Number::F64(v1), Number::F64(v2)) => Ok(Number::F64(v1/v2)),
            _ => Err("Mismatched number types for '/'".to_string())
        }
    }
}
impl std::ops::Rem for Number {
    type Output = Result<Number, String>;
    fn rem(self, other: Number) -> Result<Number, String> {
        match (self, other) {
            (Number::I8(v1), Number::I8(v2)) => Ok(Number::I8(v1%v2)),
            (Number::I16(v1), Number::I16(v2)) => Ok(Number::I16(v1%v2)),
            (Number::I32(v1), Number::I32(v2)) => Ok(Number::I32(v1%v2)),
            (Number::I64(v1), Number::I64(v2)) => Ok(Number::I64(v1%v2)),
            (Number::I128(v1), Number::I128(v2)) => Ok(Number::I128(v1%v2)),
            (Number::U8(v1), Number::U8(v2)) => Ok(Number::U8(v1%v2)),
            (Number::U16(v1), Number::U16(v2)) => Ok(Number::U16(v1%v2)),
            (Number::U32(v1), Number::U32(v2)) => Ok(Number::U32(v1%v2)),
            (Number::U64(v1), Number::U64(v2)) => Ok(Number::U64(v1%v2)),
            (Number::U128(v1), Number::U128(v2)) => Ok(Number::U128(v1%v2)),
            (Number::F32(v1), Number::F32(v2)) => Ok(Number::F32(v1%v2)),
            (Number::F64(v1), Number::F64(v2)) => Ok(Number::F64(v1%v2)),
            _ => Err("Mismatched number types for '%'".to_string())
        }
    }
}
