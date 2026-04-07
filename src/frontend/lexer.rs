use logos::Logos;
use super::token::Token;

pub fn lexer(input: &str) -> Vec<Token> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => tokens.push(token),
            Err(_) => {
                panic!("Invalid token");
            }
        }
    }

    tokens
}