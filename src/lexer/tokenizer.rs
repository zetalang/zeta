use std::sync::Arc;

use super::types::*;

pub fn tokenizer(contents: &str) -> Vec<TokenType> {
    let mut tokens = TokenParser::new(contents);
    let mut is_close_single_line = true;
    while let Some(&c) = tokens.peek() {
        if is_close_single_line {
            match c {
                '{' => tokens.push(Token::OpenBrace, "{"),
                '}' => tokens.push(Token::CloseBrace, "}"),
                '(' => tokens.push(Token::OpenParen, "("),
                ')' => tokens.push(Token::CloseParen, ")"),
                '[' => tokens.push(Token::OpenSquareParen, "["),
                ']' => tokens.push(Token::CloseSquareParen, "]"),
                ' ' | '\t' | '\r' | '\n' => tokens.drop(),
                'a'..='z' | 'A'..='Z' => {
                    let word: &str = &tokens.get_string(|x| x.is_ascii() && x.is_alphanumeric());
                    match word {
                        "async" => tokens.push_back(Token::Keyword(Keyword::Async), "async"),
                        "int" => tokens.push_back(Token::Keyword(Keyword::Int), "int"),
                        "str" => tokens.push_back(Token::Keyword(Keyword::String), "str"),
                        "fn" => tokens.push_back(Token::Keyword(Keyword::Func), "fn"),
                        "let" => tokens.push_back(Token::Keyword(Keyword::Let), "let"),
                        "mlstr" => tokens.push_back(Token::Keyword(Keyword::MLstr), "mlstr"),
                        "return" => tokens.push_back(Token::Keyword(Keyword::Return), "return"),
                        "bool" => tokens.push_back(Token::Keyword(Keyword::Bool), "bool"),
                        "if" => tokens.push_back(Token::Keyword(Keyword::If), "if"),
                        "else" => tokens.push_back(Token::Keyword(Keyword::Else), "else"),
                        "while" => tokens.push_back(Token::Keyword(Keyword::While), "while"),
                        "const" => tokens.push_back(Token::Keyword(Keyword::Const), "const"),
                        "use" => tokens.push_back(Token::Keyword(Keyword::Use), "use"),
                        "for" => tokens.push_back(Token::Keyword(Keyword::For), "for"),
                        "pub" => tokens.push_back(Token::Keyword(Keyword::Pub), "pub"),
                        _ => tokens.push_back(Token::Identifier(word.to_string()), word),
                    }
                }

                '`' => tokens.push(Token::Literal(Value::MLStr(c.into())), "`"),
                '"' => tokens.push(Token::Literal(Value::Char((c as u8).to_string())), "\""),
                '\'' => tokens.push(Token::Literal(Value::Char((c as u8).to_string())), "'"),
                '0'..='9' => {
                    let word = tokens.get_string(|x| x.is_ascii() && (x.is_digit(16) || x == &'x'));
                    let int: u32 = if word.starts_with("0x") {
                        u32::from_str_radix(&word[2..], 16).expect("Not a number")
                    } else {
                        word.parse().expect("Not a number")
                    };
                    tokens.push_back(Token::Literal(Value::Int(int)), int.to_string().as_str())
                }
                '~' => tokens.push(Token::BitComp, "~"),
                ',' => tokens.push(Token::Comma, ","),
                multi => match (tokens.next().unwrap(), tokens.peek()) {
                    ('&', Some(&'&')) => tokens.push(Token::And, "&&"),
                    ('|', Some(&'|')) => tokens.push(Token::Or, "||"),

                    ('=', Some(&'>')) => {
                        tokens.next();
                        tokens.push(Token::AsignFunc, "=>")
                    }
                    ('=', Some(&'=')) => tokens.push(Token::Equal, "=="),
                    ('<', Some(&'=')) => tokens.push(Token::LessThanOrEqual, "<="),
                    ('>', Some(&'=')) => tokens.push(Token::GreaterThanOrEqual, ">="),
                    ('!', Some(&'=')) => tokens.push(Token::NotEqual, "!="),
                    ('<', Some(&'<')) => {
                        tokens.next();
                        if let Some(&'=') = tokens.peek() {
                            tokens.push(Token::AssignBitLeft, "<<=")
                        } else {
                            tokens.push_back(Token::BitwiseLeft, "<<")
                        }
                    }
                    ('>', Some(&'>')) => {
                        tokens.next();
                        if let Some(&'=') = tokens.peek() {
                            tokens.push(Token::AssignBitRight, ">>=")
                        } else {
                            tokens.push_back(Token::BitwiseRight, ">>")
                        }
                    }
                    ('+', Some(&'=')) => tokens.push(Token::AssignAdd, "+="),
                    ('-', Some(&'=')) => tokens.push(Token::AssignSub, "-="),
                    ('*', Some(&'=')) => tokens.push(Token::AssignMul, "*="),
                    ('/', Some(&'=')) => tokens.push(Token::AssignDiv, "/="),
                    ('/', _) => {
                        let next = tokens.peek().unwrap();
                        tokens.push_back(Token::Division, "/")
                    }
                    ('%', Some(&'=')) => tokens.push(Token::AssignMod, "%="),
                    ('&', Some(&'=')) => tokens.push(Token::AssignAnd, "&="),
                    ('|', Some(&'=')) => tokens.push(Token::AssignOr, "|="),
                    ('^', Some(&'=')) => tokens.push(Token::AssignXor, "^="),
                    ('+', Some(&'+')) => tokens.push(Token::Increment, "++"),
                    ('-', Some(&'-')) => tokens.push(Token::Decrement, "--"),
                    (':', Some(&':')) => tokens.push(Token::DoubleColon, "::"),

                    ('.', _) => tokens.push_back(Token::Dot, "."),
                    ('$', _) => tokens.push_back(Token::Dollar, "$"),
                    ('#', _) => tokens.push_back(Token::HashTag, "#"),
                    ('<', _) => tokens.push_back(Token::LessThan, "<"),
                    ('>', _) => tokens.push_back(Token::GreaterThan, ">"),
                    ('!', _) => tokens.push_back(Token::LogicalNeg, "!"),
                    ('&', _) => tokens.push_back(Token::BitwiseAnd, "&"),
                    ('|', _) => tokens.push_back(Token::BitwiseOr, "|"),
                    ('=', _) => tokens.push_back(Token::Assign, "="),
                    ('+', _) => tokens.push_back(Token::Addition, "+"),
                    ('-', _) => tokens.push_back(Token::Negation, "-"),
                    ('*', _) => tokens.push_back(Token::Multiplication, "*"),
                    ('%', _) => tokens.push_back(Token::Modulus, "%"),
                    ('^', _) => tokens.push_back(Token::BitwiseXor, "^"),
                    (':', _) => tokens.push_back(Token::Colon, ":"),
                    ('?', _) => tokens.push_back(Token::Question, "?"),
                    _ => panic!("Unknown token {:?}", multi),
                },
            };
        } else {
            if c == '\n' {
                is_close_single_line = true;
            } else {
                tokens.drop();
            }
        }
    }
    tokens.tokens
}
