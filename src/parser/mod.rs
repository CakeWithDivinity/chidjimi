use std::{collections::HashMap, slice::Iter, iter::Peekable};

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

pub fn parse(tokens: &mut Peekable<Iter<Token>>) -> JsonObject {
    while let Some(token) = tokens.next() {
        match token {
            Token::Null => return JsonObject::Null,
            Token::Boolean(value) => return JsonObject::Boolean(*value),
            Token::Number(value) => return JsonObject::Number(*value),
            Token::String(value) => return JsonObject::String(value.to_string()),
            Token::OpenBracket => {
                let mut elements = vec![];

                while let Some(token) = tokens.peek() {
                    match token {
                        Token::CloseBracket => {
                            tokens.next();
                            break;
                        },
                        Token::Comma => { 
                            tokens.next();
                            continue;
                        },
                        _ => elements.push(parse(tokens)),
                    }

                }

                return JsonObject::Array(elements)
            },
            Token::OpenBrace => {
                let mut elements = HashMap::new();

                while let Some(token) = tokens.peek() {
                    match token {
                        Token::CloseBrace => {
                            tokens.next();
                            break;
                        },
                        Token::Comma => { 
                            tokens.next();
                            continue;
                        },
                        Token::String(key) => {
                            tokens.next();
                            match tokens.next() {
                                Some(Token::Colon) => {
                                    elements.insert(key.to_string(), parse(tokens));
                                },
                                _ => panic!("Expected colon after key"),
                            }
                        },
                        _ => panic!("Invalid token inside object"),
                    }
                }

                return JsonObject::Object(elements)
            },
            _ => todo!("{:?}", token),
        };
    }

    JsonObject::Null
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_empty_tokens_as_null() {
        let tokens = vec![];
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(json, JsonObject::Null);
    }

    #[test]
    fn parses_literals() {
        let tokens = vec![Token::Null];
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(json, JsonObject::Null);

        let tokens = vec![Token::Boolean(true)];
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(json, JsonObject::Boolean(true));

        let tokens = vec![Token::Number(42.69)];
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(json, JsonObject::Number(42.69));

        let tokens = vec![Token::String("Foo".to_string())];
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(json, JsonObject::String("Foo".to_string()));
    }

    #[test]
    fn parses_arrays() {
        let tokens = vec![Token::OpenBracket, Token::CloseBracket];
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(json, JsonObject::Array(vec![]));

        let tokens = vec![
            Token::OpenBracket,
            Token::Number(42.69),
            Token::CloseBracket,
        ];
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(json, JsonObject::Array(vec![JsonObject::Number(42.69)]));

        let tokens = vec![
            Token::OpenBracket,
            Token::Number(42.69),
            Token::Comma,
            Token::Number(69.42),
            Token::CloseBracket,
        ];
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(
            json,
            JsonObject::Array(vec![
                JsonObject::Number(42.69),
                JsonObject::Number(69.42)
            ])
        );

        let tokens = vec![
            Token::OpenBracket,
            Token::OpenBracket,
            Token::Number(42.69),
            Token::CloseBracket,
            Token::CloseBracket,
        ];
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(
            json,
            JsonObject::Array(vec![
                JsonObject::Array(vec![JsonObject::Number(42.69)])
            ])
        );
    }

    #[test]
    fn test_objects() {
        let tokens = vec![Token::OpenBrace, Token::CloseBrace];
        let json = parse(&mut tokens.iter().peekable());
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
        let json = parse(&mut tokens.iter().peekable());
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
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(json, JsonObject::Object(map));

        let tokens = vec![
            Token::OpenBrace,
            Token::String("foo".to_string()),
            Token::Colon,
            Token::OpenBrace,
            Token::String("bar".to_string()),
            Token::Colon,
            Token::Number(42.69),
            Token::CloseBrace,
            Token::CloseBrace,
        ];
        let mut map = HashMap::new();
        let mut inner_map = HashMap::new();
        inner_map.insert("bar".to_string(), JsonObject::Number(42.69));
        map.insert("foo".to_string(), JsonObject::Object(inner_map));
        let json = parse(&mut tokens.iter().peekable());
        assert_eq!(json, JsonObject::Object(map));
    }
}
