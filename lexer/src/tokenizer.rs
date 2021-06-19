use crate::{errors::TokenizeError, *};

pub fn tokenize(contents: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = TokenParser::new(contents);
    let mut is_close_single_line = true;
    while let Some(&c) = tokens.peek() {
        if is_close_single_line {
            match c {
                '{' => tokens.push(Token::OpenBrace),
                '}' => tokens.push(Token::CloseBrace),
                '(' => tokens.push(Token::OpenParen),
                ')' => tokens.push(Token::CloseParen),
                '[' => tokens.push(Token::OpenSquareParen),
                ']' => tokens.push(Token::CloseSquareParen),
                ' ' | '\t' | '\r' | '\n' => tokens.drop(),
                'a'..='z' | 'A'..='Z' => {
                    let word: &str = &tokens.get_string(|x| x.is_ascii() && x.is_alphanumeric());
                    match word {
                        "async" => tokens.push_back(Token::Keyword(Keyword::Async)),
                        "int" => tokens.push_back(Token::Keyword(Keyword::Int)),
                        "str" => tokens.push_back(Token::Keyword(Keyword::String)),
                        "fn" => tokens.push_back(Token::Keyword(Keyword::Func)),
                        "let" => tokens.push_back(Token::Keyword(Keyword::Let)),
                        "mlstr" => tokens.push_back(Token::Keyword(Keyword::MLstr)),
                        "return" => tokens.push_back(Token::Keyword(Keyword::Return)),
                        "void" => tokens.push_back(Token::Keyword(Keyword::Void)),
                        "bool" => tokens.push_back(Token::Keyword(Keyword::Bool)),
                        "if" => tokens.push_back(Token::Keyword(Keyword::If)),
                        "else" => tokens.push_back(Token::Keyword(Keyword::Else)),
                        "while" => tokens.push_back(Token::Keyword(Keyword::While)),
                        "const" => tokens.push_back(Token::Keyword(Keyword::Const)),
                        "use" => tokens.push_back(Token::Keyword(Keyword::Use)),
                        "for" => tokens.push_back(Token::Keyword(Keyword::For)),
                        "pub" => tokens.push_back(Token::Keyword(Keyword::Pub)),
                        _ => tokens.push_back(Token::Identifier(word.to_string())),
                    }
                }
                '`' => {
                    tokens.next();
                }
                '"' => {
                    tokens.next();
                }
                '\'' => {
                    tokens.next();
                }
                '0'..='9' => {
                    let word = tokens.get_string(|x| x.is_ascii() && (x.is_digit(16) || x == &'x'));
                    let int: u32 = if word.starts_with("0x") {
                        u32::from_str_radix(&word[2..], 16)?
                    } else {
                        word.parse()?
                    };
                    tokens.push_back(Token::Literal(Value::Int(int)))
                }
                '~' => tokens.push(Token::BitComp),
                ',' => tokens.push(Token::Comma),
                multi => match (tokens.next().unwrap(), tokens.peek()) {
                    ('&', Some(&'&')) => tokens.push(Token::And),
                    ('|', Some(&'|')) => tokens.push(Token::Or),

                    ('=', Some(&'>')) => {
                        tokens.next();
                        tokens.push(Token::AsignFunc)
                    }
                    ('=', Some(&'=')) => tokens.push(Token::Equal),
                    ('<', Some(&'=')) => tokens.push(Token::LessThanOrEqual),
                    ('>', Some(&'=')) => tokens.push(Token::GreaterThanOrEqual),
                    ('!', Some(&'=')) => tokens.push(Token::NotEqual),
                    ('<', Some(&'<')) => {
                        tokens.next();
                        if let Some(&'=') = tokens.peek() {
                            tokens.push(Token::AssignBitLeft)
                        } else {
                            tokens.push_back(Token::BitwiseLeft)
                        }
                    }
                    ('>', Some(&'>')) => {
                        tokens.next();
                        if let Some(&'=') = tokens.peek() {
                            tokens.push(Token::AssignBitRight)
                        } else {
                            tokens.push_back(Token::BitwiseRight)
                        }
                    }
                    ('+', Some(&'=')) => tokens.push(Token::AssignAdd),
                    ('-', Some(&'=')) => tokens.push(Token::AssignSub),
                    ('*', Some(&'=')) => tokens.push(Token::AssignMul),
                    ('/', Some(&'=')) => tokens.push(Token::AssignDiv),
                    ('/', _) => {
                        let next = tokens.peek().unwrap();
                        if next == &'/' {
                            tokens.next();
                            is_close_single_line = false;
                        } else {
                            tokens.push_back(Token::Division)
                        }
                    }
                    ('%', Some(&'=')) => tokens.push(Token::AssignMod),
                    ('&', Some(&'=')) => tokens.push(Token::AssignAnd),
                    ('|', Some(&'=')) => tokens.push(Token::AssignOr),
                    ('^', Some(&'=')) => tokens.push(Token::AssignXor),
                    ('+', Some(&'+')) => tokens.push(Token::Increment),
                    ('-', Some(&'-')) => tokens.push(Token::Decrement),
                    (':', Some(&':')) => tokens.push(Token::DoubleColon),

                    ('.', _) => tokens.push_back(Token::Dot),
                    ('$', _) => tokens.push_back(Token::Dollar),
                    ('#', _) => tokens.push_back(Token::HashTag),
                    ('<', _) => tokens.push_back(Token::LessThan),
                    ('>', _) => tokens.push_back(Token::GreaterThan),
                    ('!', _) => tokens.push_back(Token::LogicalNeg),
                    ('&', _) => tokens.push_back(Token::BitwiseAnd),
                    ('|', _) => tokens.push_back(Token::BitwiseOr),
                    ('=', _) => tokens.push_back(Token::Assign),
                    ('+', _) => tokens.push_back(Token::Addition),
                    ('-', _) => tokens.push_back(Token::Negation),
                    ('*', _) => tokens.push_back(Token::Multiplication),
                    ('%', _) => tokens.push_back(Token::Modulus),
                    ('^', _) => tokens.push_back(Token::BitwiseXor),
                    (':', _) => tokens.push_back(Token::Colon),
                    ('?', _) => tokens.push_back(Token::Question),
                    _ => return Err(TokenizeError::UnknownToken(multi)),
                },
            };
        } else if c == '\n' {
            is_close_single_line = true;
        } else {
            tokens.drop();
        }
    }
    Ok(tokens.tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_char_ops() {
        assert_eq!(
            tokenize("{}$").unwrap(),
            vec![Token::OpenBrace, Token::CloseBrace, Token::Dollar]
        );
    }

    #[test]
    fn multi_char_ops() {
        assert_eq!(
            tokenize("&&||>>").unwrap(),
            vec![Token::And, Token::Or, Token::BitwiseRight]
        );
    }

    #[test]
    fn drop_whitespace() {
        assert_eq!(
            tokenize("%=\r34\n   ~").unwrap(),
            vec![
                Token::AssignMod,
                Token::Literal(Value::Int(34)),
                Token::BitComp
            ]
        );
    }

    #[test]
    fn basic_identifiers() {
        assert_eq!(
            tokenize("async fn pub else").unwrap(),
            vec![
                Token::Keyword(Keyword::Async),
                Token::Keyword(Keyword::Func),
                Token::Keyword(Keyword::Pub),
                Token::Keyword(Keyword::Else)
            ]
        );
    }

    #[test]
    fn false_positive_tokens() {
        match tokenize("👫 🔫") {
            Err(TokenizeError::UnknownToken(_)) => {}
            _ => panic!(),
        }
    }
}
