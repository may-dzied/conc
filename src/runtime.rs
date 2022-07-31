use std::collections::HashMap;

use crate::{
    parser::Node,
    lexer::Token,
    value::Value,
};

const BUILTINS: &[&str] = &[
    "+", "-", "*", "/",
    "true", "false", "==", "!=", ">", "<", ">=", "<=",
    "print"
];

struct State {
    stack: Vec<Value>,
    global: HashMap<String, Value>,
    scoped: Vec<HashMap<String, Value>>
}

impl State {

    fn pop(&mut self) -> Result<Value, String> {
        self.stack
            .pop()
            .ok_or("Stack empty when popped".to_string())
    }

    fn binary_op(&mut self, function: fn(Value, Value) -> Result<Value, String>) -> Result<(), String> {
        let first = self.pop()?;
        let second = self.pop()?;
        let result = function(second, first)?;
        self.stack.push(result);
        Ok(())
    }

    fn logic_op(&mut self, function: fn(Value, Value) -> Value) -> Result<(), String> {
        let first = self.pop()?;
        let second = self.pop()?;
        let result = function(second, first);
        self.stack.push(result);
        Ok(())
    }

    fn execute_function(&mut self, function: &Vec<Node>) -> Result<(), String> {
        for node in function {
            self.execute_node(node)?;
        }
        Ok(())
    }

    fn execute_builtin(&mut self, key: &String) -> Result<(), String> {
        match key.as_str() {
            "+" => self.binary_op(|a,b| a+b)?,
            "-" => self.binary_op(|a,b| a-b)?,
            "*" => self.binary_op(|a,b| a*b)?,
            "/" => self.binary_op(|a,b| a/b)?,
            ">" => self.binary_op(|a,b| Ok(Value::Boolean(a>b)))?,
            "<" => self.binary_op(|a,b| Ok(Value::Boolean(a<b)))?,
            ">=" => self.binary_op(|a,b| Ok(Value::Boolean(a>=b)))?,
            "<=" => self.binary_op(|a,b| Ok(Value::Boolean(a<=b)))?,
            "==" => self.logic_op(|a,b| Value::Boolean(a==b))?,
            "!=" => self.logic_op(|a,b| Value::Boolean(a!=b))?,
            "true" => self.stack.push(Value::Boolean(true)),
            "false" => self.stack.push(Value::Boolean(false)),
            "print" => {
                let item = self.pop()?;
                println!("{:?}", item);
            }
            _ => return Err(format!("Unknown builtin: {}", key))
        }
        Ok(())
    }

    fn execute_node(&mut self, node: &Node) -> Result<(), String> {
        match node {
            Node::Tok(Token::Number(number)) => self.stack.push(Value::Number(*number)),
            Node::Tok(Token::String(string)) => self.stack.push(Value::String(string.clone())),
            Node::Tok(Token::OpenBracket) | Node::Tok(Token::CloseBracket) => return Err("parsing error: bracket appeared in ast".to_string()),
            Node::Group(inner) => self.stack.push(Value::Function(inner.to_vec())),
            Node::Tok(Token::Identifier(ident)) => {
                if BUILTINS.contains(&ident.as_str()) {
                    self.execute_builtin(&ident)?;
                } else {
                    let global_func = self.global.get(ident).cloned();
                    let scoped_func = self.scoped_get(ident);
                    let func = scoped_func
                        .or(global_func)
                        .ok_or(format!("Unknown function: {}", ident))?;
                    if let Value::Function(inner) = func {
                        self.execute_function(&inner)?;
                    }
                }
            },
        }
        Ok(())
    }

    fn scoped_get(&self, key: &String) -> Option<Value> {
        for scope in self.scoped.iter().rev() {
            let option = scope.get(key);
            if option.is_some() {
                return option.cloned();
            }
        }
        None
    }

}

pub fn run_tree(tree: Vec<Node>) -> Result<(), String> {
    let mut state = State {
        stack: vec![],
        global: HashMap::new(),
        scoped: vec![]
    };
    state.execute_function(&tree)?;
    Ok(())
}
