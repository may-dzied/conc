use std::{fs, env};
use conc::{lexer, parser, runtime};

fn main() -> Result<(), String> {
    let file_name = env::args()
        .nth(1)
        .ok_or("Not enough arguments provided".to_string())?;
    let text = fs::read_to_string(&file_name)
        .map_err(|_| format!("Source file not found: '{}'", file_name))?;
    let tokens = lexer::lex(text)?;
    let tree = parser::parse(tokens)?;
    runtime::run_tree(tree)?;
    Ok(())
}
