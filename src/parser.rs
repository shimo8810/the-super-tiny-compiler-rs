use std::iter::Peekable;
use std::vec::IntoIter;

use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum AstNode {
    Program { body: Vec<AstNode> },
    CallExpression { name: String, params: Vec<AstNode> },
    NumberLiteral(String),
    StringLiteral(String),
}

fn walk(token: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Result<AstNode, String> {
    match token {
        Token::Number(n) => Ok(AstNode::NumberLiteral(n)),
        Token::String(s) => Ok(AstNode::StringLiteral(s)),
        Token::Paren(p) if p == "(" => {
            let name = match tokens.next() {
                Some(Token::Name(name)) => name,
                _ => return Err("missing 'name' after parent '('".into()),
            };

            let mut params = vec![];

            while !matches!(tokens.peek(), Some(Token::Paren(p)) if p == ")") {
                match walk(tokens.next().unwrap(), tokens) {
                    Ok(node) => params.push(node),
                    Err(s) => return Err(s),
                }
            }
            tokens.next().unwrap();
            Ok(AstNode::CallExpression { name, params })
        }
        _ => Err("I dont know this token".into()),
    }
}
pub fn parser(tokens: Vec<Token>) -> Result<AstNode, String> {
    let mut tokens = tokens.into_iter().peekable();

    let mut body = vec![];

    while let Some(token) = tokens.next() {
        match walk(token, &mut tokens) {
            Ok(nodes) => body.push(nodes),
            Err(value) => return Err(value),
        }
    }

    Ok(AstNode::Program { body })
}
