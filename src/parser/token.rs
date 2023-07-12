use std::{iter::Peekable, num::ParseFloatError, str::Chars};

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidNumber(ParseFloatError),
    UnexpectedEndOfInput,
    InvalidToken,
}

pub fn tokenize(input: String) -> Result<Vec<Token>, ParseError> {
    let mut input = input.chars().peekable();
    let mut tokens = vec![];

    while let Some(char) = input.next() {
        let token = match char {
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '"' => {
                // TODO: catch undetermined strings
                let mut string = String::new();
                for char in input.by_ref() {
                    match char {
                        '"' => break,
                        _ => string.push(char),
                    }
                }

                Token::String(string)
            }
            '0'..='9' => {
                let mut number = String::new();
                number.push(char);

                while let Some(&char) = input.peek() {
                    match char {
                        '0'..='9' | '.' | 'e' | 'E' => {
                            number.push(char);
                            input.next();
                        }
                        _ => break,
                    }
                }

                Token::Number(number.parse().map_err(ParseError::InvalidNumber)?)
            }
            't' => assert_next_chars(&mut input, "rue").map(|_| Token::Boolean(true))?,
            'f' => assert_next_chars(&mut input, "alse").map(|_| Token::Boolean(false))?,
            'n' => assert_next_chars(&mut input, "ull").map(|_| Token::Null)?,
            ' ' | '\n' | '\t' => continue,
            _ => return Err(ParseError::InvalidToken),
        };

        tokens.push(token);
    }

    Ok(tokens)
}

fn assert_next_chars(input: &mut Peekable<Chars>, expected: &str) -> Result<(), ParseError> {
    let mut next_chars = vec![];
    for _ in 0..expected.len() {
        next_chars.push(input.next().ok_or(ParseError::UnexpectedEndOfInput)?);
    }

    if next_chars.iter().collect::<String>().as_str() == expected {
        Ok(())
    } else {
        Err(ParseError::UnexpectedEndOfInput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_symbols() {
        let input = "{}[]:,";

        let tokens = tokenize(input.to_string()).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::OpenBrace,
                Token::CloseBrace,
                Token::OpenBracket,
                Token::CloseBracket,
                Token::Colon,
                Token::Comma,
            ]
        );
    }

    #[test]
    fn test_tokenize_string() {
        let input = r#""hello world""#;
        let tokens = tokenize(input.to_string()).unwrap();
        assert_eq!(tokens, vec![Token::String("hello world".to_string())]);
    }

    #[test]
    fn test_tokenize_number() {
        let input = "123.456";
        let tokens = tokenize(input.to_string()).unwrap();
        assert_eq!(tokens, vec![Token::Number(123.456)]);

        let input = "123.456e2";
        let tokens = tokenize(input.to_string()).unwrap();
        assert_eq!(tokens, vec![Token::Number(12345.6)]);
    }

    #[test]
    fn test_tokenize_literals() {
        let input = " truefalsenull";

        let tokens = tokenize(input.to_string()).unwrap();

        assert_eq!(
            tokens,
            vec![Token::Boolean(true), Token::Boolean(false), Token::Null]
        );
    }

    #[test]
    fn test_tokenize_errors() {
        let input = "123.456.789";
        let tokens = tokenize(input.to_string());
        assert!(matches!(tokens, Err(ParseError::InvalidNumber(_))));

        let input = "a";
        let tokens = tokenize(input.to_string());
        assert!(matches!(tokens, Err(ParseError::InvalidToken)));

        let input = "tru";
        let tokens = tokenize(input.to_string());
        assert!(matches!(tokens, Err(ParseError::UnexpectedEndOfInput)));
    }
}
