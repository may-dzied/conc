use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Tok(Token),
    Group(Vec<Node>)
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, String> {

    let mut result = vec![];
    let mut working = vec![];
    let mut level = 0;

    for token in tokens {
        match token {
            Token::OpenBracket => {
                level += 1;
                if level > 1 {
                    working.push(token);
                }
            },
            Token::CloseBracket => {
                level -= 1;
                if level == 0 {
                    let inner = parse(working)?;
                    result.push(Node::Group(inner));
                    working = vec![];
                } else {
                    working.push(token);
                }
            },
            _ => {
                if level == 0 {
                    result.push(Node::Tok(token));
                } else {
                    working.push(token);
                }
            }
        }
    }

    Ok(result)
}
