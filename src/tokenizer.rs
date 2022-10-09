#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Paren(String),
    String(String),
    Number(String),
    Name(String),
}

#[derive(Debug)]
pub enum TokenizeError {
    TypeError(String),
}

pub fn tokenizer(input: &str) -> Result<Vec<Token>, TokenizeError> {
    //
    let mut input = input.chars().peekable();
    let mut tokens = vec![];

    while let Some(c) = input.next() {
        match c {
            '(' | ')' => tokens.push(Token::Paren(c.to_string())),
            'a'..='z' => {
                let mut s = vec![];
                s.push(c);
                while let Some(p) = input.peek() {
                    if ('a'..='z').contains(p) {
                        s.push(input.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Name(s.into_iter().collect()));
            }
            '0'..='9' => {
                let mut s = vec![];
                s.push(c);
                while let Some(p) = input.peek() {
                    if ('0'..='9').contains(p) {
                        s.push(input.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(s.into_iter().collect()));
            }
            '"' => {
                // string の実装
                let mut s = vec![];

                for p in input.by_ref() {
                    if p == '"' {
                        break;
                    }
                    s.push(p);
                }
                tokens.push(Token::String(s.into_iter().collect()));
            }
            ' ' | '\t' | '\n' | '\r' => {
                // 空白文字は無視
            }
            _ => {
                println!("other char");

                return Err(TokenizeError::TypeError(format!(
                    "I dont know what this character is: {}",
                    c
                )));
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_case1() {
        let s = "(add 2 (subtract 4 2))";
        let ans_tokens = vec![
            Token::Paren("(".to_string()),
            Token::Name("add".to_string()),
            Token::Number("2".to_string()),
            Token::Paren("(".to_string()),
            Token::Name("subtract".to_string()),
            Token::Number("4".to_string()),
            Token::Number("2".to_string()),
            Token::Paren(")".to_string()),
            Token::Paren(")".to_string()),
        ];
        let tokens = tokenizer(s);
        assert_eq!(tokens.unwrap(), ans_tokens);
    }
}
