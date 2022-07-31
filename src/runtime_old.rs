use std::collections;
use crate::{
    lexer::{lex, Token},
    parser::{parse, Node},
    value::Value,
    number::Number
};

#[derive(Clone)]
struct State {
    stack: Vec<Value>,
    global_variables: collections::HashMap<String, Value>,
    scoped_variables: Vec<collections::HashMap<String, Value>>
}

impl State {
    fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or("Nothing to pop".to_string())
    }
    fn scoped_get(&mut self, key: &String) -> Result<Value, String> {
        let mut found_result = None;
        let mut popped_scopes = vec![];
        let mut scope;
        while found_result.is_none() {
            scope = self.scoped_variables.pop().ok_or("Ran out of scopes".to_string())?;
            popped_scopes.push(scope.clone());
            found_result = scope.get(key);
        }
        for scope in popped_scopes.iter().rev() {
            self.scoped_variables.push(scope.clone());
        }
        found_result.cloned().ok_or("Scoped variable not found".to_string())
    }
}

fn binary_op(state: &mut State, func: fn(Value, Value) -> Result<Value, String>) -> Result<(), String> {
    let first = state.pop()?;
    let second = state.pop()?;
    state.stack.push(func(first, second)?);
    Ok(())
}

fn scoped_execute(tree: &Vec<Node>, state: &mut State) -> Result<(), String> {
    state.scoped_variables.push(collections::HashMap::new());
    for node in tree {
        execute_node(node, state)?;
    }
    state.scoped_variables.pop().ok_or("No scope to pop".to_string())?;
    Ok(())
}

fn execute_node(node: &Node, state: &mut State) -> Result<(), String> {
    match node {
        Node::Group(inner) => state.stack.push(Value::Function(inner.to_vec())),
        Node::Tok(token) => {
            match token {
                Token::Number(value) => state.stack.push(Value::Number(*value)),
                Token::String(value) => state.stack.push(Value::String((*value).clone())),
                Token::Identifier(key) => {
                    match key.as_ref() {
                        "true" => state.stack.push(Value::Boolean(true)),
                        "false" => state.stack.push(Value::Boolean(false)),
                        "cast" => {
                            let result_type = state.pop()?;
                            let item = state.pop()?;
                            if let Value::String(name) = result_type {
                                let result = item.cast_to(name)?;
                                state.stack.push(result);
                            }
                        },
                        "use" => {
                            let source = state.pop()?;
                            if let Value::String(name) = source {
                                let text = std::fs::read_to_string(name).map_err(|_| "Couldn't read file in use".to_string())?;
                                let tokens = lex(text)?;
                                let tree = parse(tokens)?;
                                scoped_execute(&tree, state)?;
                            } else {
                                return Err("Argument to 'use' must be string".to_string());
                            }
                        },
                        "take" => {
                            let idx = state.pop()?;
                            if let Value::Number(Number::U64(i)) = idx {
                                let item = state.stack.remove(
                                    state.stack.len()-1-(i as usize)
                                );
                                state.stack.push(item);
                            }
                        },
                        "clone" => {
                            let top = state.pop()?;
                            state.stack.push(top.clone());
                            state.stack.push(top);
                        },
                        "swap" => {
                            let first = state.pop()?;
                            let second = state.pop()?;
                            state.stack.push(first);
                            state.stack.push(second);
                        },
                        "delete" => {
                            state.pop()?;
                        },
                        "insert" => {
                            let index = state.pop()?;
                            let item = state.pop()?;
                            let vector = state.pop()?;
                            if let (Value::Vector(vec), Value::Number(Number::U64(idx))) = (vector, index) {
                                let mut new_vec = vec.clone();
                                new_vec.insert(new_vec.len() - (idx as usize), item);
                                state.stack.push(Value::Vector(new_vec));
                            } else {
                                return Err("Invalid types for insert".to_string());
                            }
                        },
                        "remove" => {
                            let index = state.pop()?;
                            let vector = state.pop()?;
                            if let (Value::Vector(mut vec), Value::Number(Number::U64(idx))) = (vector, index) {
                                let item = vec.remove(vec.len() - (idx as usize) - 1);
                                state.stack.push(item);
                            }
                        },
                        "global_bind" => {
                            let name = state.pop()?;
                            let binding = state.pop()?;
                            if let Value::String(bind_name) = name {
                                state.global_variables.insert(bind_name, binding);
                            } else {
                                return Err("Must be bound to string".to_string());
                            }
                        },
                        "global_get" => {
                            let name = state.pop()?;
                            if let Value::String(var_name) = name {
                                let item = state.global_variables.get(&var_name).ok_or(format!("Variable not found: '{}'", var_name))?;
                                state.stack.push(item.clone());
                            } else {
                                return Err("Get key must be string".to_string());
                            }
                        },
                        "scoped_bind" => {
                            let name = state.pop()?;
                            let binding = state.pop()?;
                            if let Value::String(bind_name) = name {
                                let mut current_scope = state.scoped_variables.pop().ok_or("Interpreter error: ran out of scope".to_string())?;
                                current_scope.insert(bind_name, binding);
                                state.scoped_variables.push(current_scope);
                            } else {
                                return Err("Must be bound to string".to_string());
                            }
                        },
                        "scoped_get" => {
                            let name = state.pop()?;
                            if let Value::String(var_name) = name {
                                let item = state.scoped_get(&var_name)?;
                                state.stack.push(item);
                            } else {
                                return Err("Get key must be string".to_string());
                            }
                        },
                        "into_vec" => {
                            let function = state.pop()?;
                            if let Value::Function(inner) = function {
                                let start_length = state.stack.len();
                                let mut result = collections::VecDeque::new();
                                let mut state_clone = state.clone();
                                scoped_execute(&inner, &mut state_clone)?;
                                let items_added = state_clone.stack.len() - start_length;
                                for _ in 0..items_added {
                                    let item = state_clone.pop()?;
                                    result.push_front(item);
                                }
                                state.stack.push(Value::Vector(
                                    Vec::from_iter(result.iter().map(|x| (*x).clone()))
                                ));
                            } else {
                                return Err("into_vec requires function".to_string());
                            }
                        },
                        "print" => {
                            let top = state.pop()?;
                            println!("{:?}", top);
                        },
                        "debug" => {
                            println!("{:?}", state.stack);
                        },
                        "repeat" => {
                            let function = state.pop()?;
                            let count = state.pop()?;
                            if let (Value::Function(inner), Value::Number(Number::U64(times))) = (function, count) {
                                for _ in 0..times {
                                    scoped_execute(&inner, state)?;
                                }
                            } else {
                                return Err("Wrong types for repeat".to_string());
                            }
                        },
                        "for" => {
                            let function = state.pop()?;
                            let vector = state.pop()?;
                            if let (Value::Function(inner), Value::Vector(items)) = (function, vector) {
                                for item in items {
                                    state.stack.push(item);
                                    scoped_execute(&inner, state)?;
                                }
                            } else {
                                return Err("Wrong types for for".to_string());
                            }
                        },
                        "if" => {
                            let function = state.pop()?;
                            let condition = state.pop()?;
                            if let (Value::Function(inner), Value::Boolean(val)) = (function, condition) {
                                if val {
                                    scoped_execute(&inner, state)?;
                                }
                            } else {
                                return Err("Wrong types for if".to_string());
                            }
                        },
                        "call" => {
                            let function = state.pop()?;
                            if let Value::Function(inner) = function {
                                scoped_execute(&inner, state)?;
                            }
                        },
                        "+" => binary_op(state, |a, b| a+b)?,
                        "-" => binary_op(state, |a, b| b-a)?,
                        "*" => binary_op(state, |a, b| a*b)?,
                        "/" => binary_op(state, |a, b| b/a)?,
                        "%" => binary_op(state, |a, b| b%a)?,
                        "==" => binary_op(state, |a, b| Ok(Value::Boolean(a==b)))?,
                        _ => {
                            let global_fn = state.global_variables.get(key);
                            let scoped_fn = state.scoped_variables[state.scoped_variables.len()-1].get(key);
                            let function = global_fn.or(scoped_fn).ok_or(format!("Unknown function: '{}'", key))?;

                            if let Value::Function(inner) = function.clone() {
                                scoped_execute(&inner, state)?;
                            } else {
                                return Err("Cannot call type other than function".to_string())
                            }
                        }
                    }
                }
                _ => return Err(format!("unknown {:?}", token))
            }
        }
    }
    Ok(())
}

pub fn run(tree: Vec<Node>) -> Result<(), String> {
    let mut state = State {
        stack: vec![],
        global_variables: collections::HashMap::new(),
        scoped_variables: vec![collections::HashMap::new()]
    };
    for node in tree {
        execute_node(&node, &mut state)?;
    }
    Ok(())
}
