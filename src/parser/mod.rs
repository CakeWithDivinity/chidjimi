use std::{collections::HashMap, iter::Peekable, slice::Iter};

use self::token::Token;

pub mod token;

#[derive(Debug, PartialEq)]
pub enum JsonObject {
    Object(HashMap<String, JsonObject>),
    Array(Vec<JsonObject>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub fn parse_tokens(tokens: Vec<Token>) -> JsonObject {
    parse(&mut tokens.iter().peekable())
}

fn parse(tokens: &mut Peekable<Iter<Token>>) -> JsonObject {
    match tokens.next() {
        Some(token) => {
            match token {
                Token::Null => JsonObject::Null,
                Token::Boolean(value) => JsonObject::Boolean(*value),
                Token::Number(value) => JsonObject::Number(*value),
                Token::String(value) => JsonObject::String(value.to_string()),
                Token::OpenBracket => parse_array(tokens),
                Token::OpenBrace => parse_object(tokens),
                _ => todo!("{:?}", token),
            }
        }
        None => JsonObject::Null,
    }
}

fn parse_array(tokens: &mut Peekable<Iter<Token>>) -> JsonObject {
    let mut elements = vec![];

    while let Some(token) = tokens.peek() {
        match token {
            Token::CloseBracket => {
                tokens.next();
                break;
            }
            Token::Comma => {
                tokens.next();
                continue;
            }
            _ => elements.push(parse(tokens)),
        }
    }

    JsonObject::Array(elements)
}

fn parse_object(tokens: &mut Peekable<Iter<Token>>) -> JsonObject {
    let mut elements = HashMap::new();

    while let Some(token) = tokens.peek() {
        match token {
            Token::CloseBrace => {
                tokens.next();
                break;
            }
            Token::Comma => {
                tokens.next();
                continue;
            }
            Token::String(key) => {
                tokens.next();
                elements.insert(key.to_string(), parse_object_entry(tokens));
            }
            Token::Number(value) => {
                tokens.next();
                elements.insert(value.to_string(), parse_object_entry(tokens));
            }
            Token::Boolean(value) => {
                tokens.next();
                elements.insert(value.to_string(), parse_object_entry(tokens));
            }
            _ => panic!("Invalid token inside object"),
        }
    }

    JsonObject::Object(elements)
}

fn parse_object_entry(tokens: &mut Peekable<Iter<Token>>) -> JsonObject {
    match tokens.next() {
        Some(Token::Colon) => {
            let value = parse(tokens);
            value
        }
        _ => panic!("Expected colon after key"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_empty_tokens_as_null() {
        let tokens = vec![];
        let json = parse_tokens(tokens);
        assert_eq!(json, JsonObject::Null);
    }

    #[test]
    fn parses_literals() {
        let tokens = vec![Token::Null];
        let json = parse_tokens(tokens);
        assert_eq!(json, JsonObject::Null);

        let tokens = vec![Token::Boolean(true)];
        let json = parse_tokens(tokens);
        assert_eq!(json, JsonObject::Boolean(true));

        let tokens = vec![Token::Number(42.69)];
        let json = parse_tokens(tokens);
        assert_eq!(json, JsonObject::Number(42.69));

        let tokens = vec![Token::String("Foo".to_string())];
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(json, JsonObject::String("Foo".to_string()));
    }

    #[test]
    fn parses_arrays() {
        let tokens = vec![Token::OpenBracket, Token::CloseBracket];
        let json = parse_tokens(tokens);
        assert_eq!(json, JsonObject::Array(vec![]));

        let tokens = vec![
            Token::OpenBracket,
            Token::Number(42.69),
            Token::CloseBracket,
        ];
        let json = parse_tokens(tokens);
        assert_eq!(json, JsonObject::Array(vec![JsonObject::Number(42.69)]));

        let tokens = vec![
            Token::OpenBracket,
            Token::Number(42.69),
            Token::Comma,
            Token::Number(69.42),
            Token::CloseBracket,
        ];
        let json = parse_tokens(tokens);
        assert_eq!(
            json,
            JsonObject::Array(vec![JsonObject::Number(42.69), JsonObject::Number(69.42)])
        );

        let tokens = vec![
            Token::OpenBracket,
            Token::OpenBracket,
            Token::Number(42.69),
            Token::CloseBracket,
            Token::CloseBracket,
        ];
        let json = parse_tokens(tokens);
        assert_eq!(
            json,
            JsonObject::Array(vec![JsonObject::Array(vec![JsonObject::Number(42.69)])])
        );
    }

    #[test]
    fn test_objects() {
        let tokens = vec![Token::OpenBrace, Token::CloseBrace];
        let json = parse_tokens(tokens);
        assert_eq!(json, JsonObject::Object(HashMap::new()));

        let tokens = vec![
            Token::OpenBrace,
            Token::String("foo".to_string()),
            Token::Colon,
            Token::Number(42.69),
            Token::CloseBrace,
        ];
        let mut map = HashMap::new();
        map.insert("foo".to_string(), JsonObject::Number(42.69));
        let json = parse_tokens(tokens);
        assert_eq!(json, JsonObject::Object(map));

        let tokens = vec![
            Token::OpenBrace,
            Token::String("foo".to_string()),
            Token::Colon,
            Token::Number(42.69),
            Token::Comma,
            Token::String("bar".to_string()),
            Token::Colon,
            Token::Number(69.42),
            Token::CloseBrace,
        ];
        let mut map = HashMap::new();
        map.insert("foo".to_string(), JsonObject::Number(42.69));
        map.insert("bar".to_string(), JsonObject::Number(69.42));
        let json = parse_tokens(tokens);
        assert_eq!(json, JsonObject::Object(map));

        let tokens = vec![
            Token::OpenBrace,
            Token::String("foo".to_string()),
            Token::Colon,
            Token::OpenBrace,
            Token::Boolean(false),
            Token::Colon,
            Token::Number(42.69),
            Token::CloseBrace,
            Token::CloseBrace,
        ];
        let mut map = HashMap::new();
        let mut inner_map = HashMap::new();
        inner_map.insert("false".to_string(), JsonObject::Number(42.69));
        map.insert("foo".to_string(), JsonObject::Object(inner_map));
        let json = parse_tokens(tokens);
        assert_eq!(json, JsonObject::Object(map));
    }
}
