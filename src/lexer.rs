use crate::number::Number;

const WHITESPACE: &str = " \t\n";

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    Identifier(String),
    String(String),
    Number(Number)
}

macro_rules! parse_num {
    ($builder:expr, $result:expr, $result_type:ty, $variant:ident) => {
        let text = &$builder[stringify!($result_type).len()+1..];
        let value: $result_type = text.parse().map_err(|_| format!("Couldnt parse {}: {}", stringify!($result_type), text))?;
        $result.push(Token::Number(Number::$variant(value)));
        $builder = String::new();
    };
}

pub fn lex(text: String) -> Result<Vec<Token>, String> {

    let mut result = vec![];
    let mut builder = String::new();
    let lex_text = text + "\n";
    let mut chars = lex_text.chars();

    while let Some(next) = chars.next() {
        if WHITESPACE.contains(next) && builder.trim() != "" {
            builder = builder.trim().to_string();
            if builder.starts_with("i8_") {
                parse_num!(builder, result, i8, I8);
            } else if builder.starts_with("i16_") {
                parse_num!(builder, result, i16, I16);
            } else if builder.starts_with("i32_") {
                parse_num!(builder, result, i32, I32);
            } else if builder.starts_with("i64_") {
                parse_num!(builder, result, i64, I64);
            } else if builder.starts_with("i128_") {
                parse_num!(builder, result, i128, I128);
            } else if builder.starts_with("u8_") {
                parse_num!(builder, result, u8, U8);
            } else if builder.starts_with("u16_") {
                parse_num!(builder, result, u16, U16);
            } else if builder.starts_with("u32_") {
                parse_num!(builder, result, u32, U32);
            } else if builder.starts_with("u64_") {
                parse_num!(builder, result, u64, U64);
            } else if builder.starts_with("u128_") {
                parse_num!(builder, result, u128, U128);
            } else if builder.starts_with("f32_") {
                parse_num!(builder, result, f32, F32);
            } else if builder.starts_with("f64_") {
                parse_num!(builder, result, f64, F64);
            } else if let Ok(val) = builder.parse::<u64>() {
                result.push(Token::Number(Number::U64(val)));
                builder = String::new();
            } else if let Ok(val) = builder.parse::<f64>() {
                result.push(Token::Number(Number::F64(val)));
                builder = String::new();
            } else if builder == "{" {
                result.push(Token::OpenBracket);
                builder = String::new();
            } else if builder == "}" {
                result.push(Token::CloseBracket);
                builder = String::new();
            } else {
                result.push(Token::Identifier(builder));
                builder = String::new();
            }
        } else if next == '"' {
            let mut string_char = chars.next()
                .ok_or_else(|| "String not closed".to_string())?;
            while string_char != '"' {
                builder.push(string_char);
                string_char = chars.next()
                    .ok_or_else(|| "String not closed".to_string())?;
            }
            result.push(Token::String(builder));
            builder = String::new();
        } else if next == ';' {
            while chars.next().unwrap() != ';' {}
        } else {
            builder.push(next);
        }
    }

    Ok(result)
}
