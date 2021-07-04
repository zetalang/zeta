use crate::{errors::TokenizeError, *};

pub fn tokenize(contents: &str, fname: &str) -> Result<Vec<TokenType>, TokenizeError> {
    let mut tokens = TokenParser::new(contents);
    let mut linenum = 1;
    while let Some(&c) = tokens.peek() {
        match c {
            '\n' => {
                linenum += 1;
                tokens.t_drop()
            }
            '{' => tokens.push(TokenType {
                token: Token::OpenBrace,
                val: String::from("{"),
                linenum,
            }),
            '}' => tokens.push(TokenType {
                token: Token::CloseBrace,
                val: String::from("}"),
                linenum,
            }),
            '(' => tokens.push(TokenType {
                token: Token::OpenParen,
                val: String::from("("),
                linenum,
            }),
            ')' => tokens.push(TokenType {
                token: Token::CloseParen,
                val: String::from(")"),
                linenum,
            }),
            '[' => tokens.push(TokenType {
                token: Token::OpenSquareParen,
                val: String::from("["),
                linenum,
            }),
            ']' => tokens.push(TokenType {
                token: Token::CloseSquareParen,
                val: String::from("]"),
                linenum,
            }),
            ' ' | '\t' | '\r' => tokens.t_drop(),
            'a'..='z' | 'A'..='Z' => {
                let word: &str = &tokens.get_string(|x| x.is_ascii() && x.is_alphanumeric());
                match word {
                    "async" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Async),
                        val: String::from("async"),
                        linenum,
                    }),
                    "true" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::True),
                        val: String::from("true"),
                        linenum,
                    }),
                    "false" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::False),
                        val: String::from("async"),
                        linenum,
                    }),
                    "int" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Int),
                        val: String::from("int"),
                        linenum,
                    }),
                    "str" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::String),
                        val: String::from("str"),
                        linenum,
                    }),
                    "fn" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Func),
                        val: String::from("fn"),
                        linenum,
                    }),
                    "let" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Let),
                        val: String::from("let"),
                        linenum,
                    }),
                    "mlstr" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::MLstr),
                        val: String::from("mlstr"),
                        linenum,
                    }),
                    "return" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Return),
                        val: String::from("return"),
                        linenum,
                    }),
                    "void" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Void),
                        val: String::from("void"),
                        linenum,
                    }),
                    "bool" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Bool),
                        val: String::from("bool"),
                        linenum,
                    }),
                    "if" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::If),
                        val: String::from("if"),
                        linenum,
                    }),
                    "else" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Else),
                        val: String::from("else"),
                        linenum,
                    }),
                    "while" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::While),
                        val: String::from("while"),
                        linenum,
                    }),
                    "const" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Const),
                        val: String::from("const"),
                        linenum,
                    }),
                    "use" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Use),
                        val: String::from("use"),
                        linenum,
                    }),
                    "for" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::For),
                        val: String::from("for"),
                        linenum,
                    }),
                    "pub" => tokens.push_back(TokenType {
                        token: Token::Keyword(Keyword::Pub),
                        val: String::from("pub"),
                        linenum,
                    }),
                    other => tokens.push_back(TokenType {
                        token: Token::Identifier(word.to_string()),
                        val: String::from(other),
                        linenum,
                    }),
                }
            }
            '`' => {
                tokens.p_next();
                let t = tokens.get_string(|s| (s != &'`'));
                tokens.push(TokenType {
                    token: Token::Literal(Value::MLStr(t)),
                    val: String::from("`"),
                    linenum,
                })
            }
            '"' => {
                tokens.p_next();
                let t = tokens.get_string(|s| (s != &'"'));
                tokens.push(TokenType {
                    token: Token::Literal(Value::Char(t)),
                    val: String::from('"'),
                    linenum,
                })
            }
            '\'' => {
                tokens.p_next();
                let t = tokens.get_string(|s| (s != &'\''));
                tokens.push(TokenType {
                    token: Token::Literal(Value::Char(t)),
                    val: String::from("'"),
                    linenum,
                })
            }
            '0'..='9' => {
                let word = tokens.get_string(|x| x.is_ascii() && (x.is_digit(16) || x == &'x'));

                #[allow(clippy::manual_strip)]
                let int: u64 = if word.starts_with("0x") {
                    u64::from_str_radix(&word[2..], 16)?
                } else {
                    word.parse()?
                };
                tokens.push_back(TokenType {
                    token: Token::Literal(Value::Int(int)),
                    val: int.to_string(),
                    linenum,
                })
            }
            '~' => tokens.push(TokenType {
                token: Token::BitComp,
                val: String::from('~'),
                linenum,
            }),
            ',' => tokens.push(TokenType {
                token: Token::Comma,
                val: String::from(","),
                linenum,
            }),
            multi => match (tokens.p_next().unwrap(), tokens.peek()) {
                ('&', Some(&'&')) => tokens.push(TokenType {
                    token: Token::And,
                    val: String::from("&&"),
                    linenum,
                }),
                ('|', Some(&'|')) => tokens.push(TokenType {
                    token: Token::Or,
                    val: String::from("||"),
                    linenum,
                }),

                ('=', Some(&'>')) => {
                    tokens.p_next();
                    tokens.push(TokenType {
                        token: Token::AsignFunc,
                        val: String::from("=>"),
                        linenum,
                    })
                }
                ('=', Some(&'=')) => tokens.push(TokenType {
                    token: Token::Equal,
                    val: String::from(","),
                    linenum,
                }),
                ('<', Some(&'=')) => tokens.push(TokenType {
                    token: Token::LessThanOrEqual,
                    val: String::from("<="),
                    linenum,
                }),
                ('>', Some(&'=')) => tokens.push(TokenType {
                    token: Token::GreaterThanOrEqual,
                    val: String::from(">="),
                    linenum,
                }),
                ('!', Some(&'=')) => tokens.push(TokenType {
                    token: Token::NotEqual,
                    val: String::from("!="),
                    linenum,
                }),
                ('<', Some(&'<')) => {
                    tokens.p_next();
                    if let Some(&'=') = tokens.peek() {
                        tokens.push(TokenType {
                            token: Token::AssignBitLeft,
                            val: String::from("<<="),
                            linenum,
                        })
                    } else {
                        tokens.push_back(TokenType {
                            token: Token::BitwiseLeft,
                            val: String::from("<<"),
                            linenum,
                        })
                    }
                }
                ('>', Some(&'>')) => {
                    tokens.p_next();
                    if let Some(&'=') = tokens.peek() {
                        tokens.push(TokenType {
                            token: Token::AssignBitRight,
                            val: String::from(">>="),
                            linenum,
                        })
                    } else {
                        tokens.push_back(TokenType {
                            token: Token::BitwiseRight,
                            val: String::from(">>"),
                            linenum,
                        })
                    }
                }
                ('+', Some(&'=')) => tokens.push(TokenType {
                    token: Token::AssignAdd,
                    val: String::from("+="),
                    linenum,
                }),
                ('-', Some(&'=')) => tokens.push(TokenType {
                    token: Token::AssignSub,
                    val: String::from("-="),
                    linenum,
                }),
                ('*', Some(&'=')) => tokens.push(TokenType {
                    token: Token::AssignMul,
                    val: String::from("*="),
                    linenum,
                }),
                ('/', Some(&'=')) => tokens.push(TokenType {
                    token: Token::AssignDiv,
                    val: String::from("/="),
                    linenum,
                }),
                ('/', _) => {
                    let next = tokens.peek().unwrap();
                    tokens.push_back(TokenType {
                        token: Token::Division,
                        val: String::from("/"),
                        linenum,
                    })
                }
                ('%', Some(&'=')) => tokens.push(TokenType {
                    token: Token::AssignMod,
                    val: String::from("%="),
                    linenum,
                }),
                ('&', Some(&'=')) => tokens.push(TokenType {
                    token: Token::AssignAnd,
                    val: String::from("&="),
                    linenum,
                }),
                ('|', Some(&'=')) => tokens.push(TokenType {
                    token: Token::AssignOr,
                    val: String::from("|="),
                    linenum,
                }),
                ('^', Some(&'=')) => tokens.push(TokenType {
                    token: Token::AssignXor,
                    val: String::from("^="),
                    linenum,
                }),
                ('+', Some(&'+')) => tokens.push(TokenType {
                    token: Token::Increment,
                    val: String::from("++"),
                    linenum,
                }),
                ('-', Some(&'-')) => tokens.push(TokenType {
                    token: Token::Decrement,
                    val: String::from("--"),
                    linenum,
                }),
                (':', Some(&':')) => tokens.push(TokenType {
                    token: Token::DoubleColon,
                    val: String::from("::"),
                    linenum,
                }),

                ('.', _) => tokens.push_back(TokenType {
                    token: Token::Dot,
                    val: String::from("."),
                    linenum,
                }),
                ('$', _) => tokens.push_back(TokenType {
                    token: Token::Dollar,
                    val: String::from("$"),
                    linenum,
                }),
                ('#', _) => tokens.push_back(TokenType {
                    token: Token::HashTag,
                    linenum,
                    val: String::from("#"),
                }),
                ('<', _) => tokens.push_back(TokenType {
                    token: Token::LessThan,
                    val: String::from("<"),
                    linenum,
                }),
                ('>', _) => tokens.push_back(TokenType {
                    token: Token::GreaterThan,
                    val: String::from(">"),
                    linenum,
                }),
                ('!', _) => tokens.push_back(TokenType {
                    token: Token::LogicalNeg,
                    val: String::from("!"),
                    linenum,
                }),
                ('&', _) => tokens.push_back(TokenType {
                    linenum,
                    token: Token::BitwiseAnd,
                    val: String::from("&"),
                }),
                ('|', _) => tokens.push_back(TokenType {
                    token: Token::BitwiseOr,
                    linenum,
                    val: String::from("|"),
                }),
                ('=', _) => tokens.push_back(TokenType {
                    token: Token::Assign,
                    val: String::from("="),
                    linenum,
                }),
                ('+', _) => tokens.push_back(TokenType {
                    token: Token::Addition,
                    val: String::from("+"),
                    linenum,
                }),
                ('-', _) => tokens.push_back(TokenType {
                    token: Token::Negation,
                    val: String::from("-"),
                    linenum,
                }),
                ('*', _) => tokens.push_back(TokenType {
                    token: Token::Multiplication,
                    val: String::from("*"),
                    linenum,
                }),
                ('%', _) => tokens.push_back(TokenType {
                    token: Token::Modulus,
                    val: String::from("%"),
                    linenum,
                }),
                ('^', _) => tokens.push_back(TokenType {
                    token: Token::BitwiseXor,
                    val: String::from("^"),
                    linenum,
                }),
                (':', _) => tokens.push_back(TokenType {
                    token: Token::Colon,
                    val: String::from(":"),
                    linenum,
                }),
                ('?', _) => tokens.push_back(TokenType {
                    token: Token::Question,
                    val: String::from("?"),
                    linenum,
                }),
                _ => return Err(TokenizeError::UnknownToken{c: multi, linenum, filename: fname.into()}),
            },
        };
    }

    Ok(tokens.tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_char_ops() {
        assert_eq!(
            tokenize("{}$", "").unwrap(),
            vec![
                TokenType {
                    token: Token::OpenBrace,
                    val: String::from("{"),
                    linenum: 1
                },
                TokenType {
                    token: Token::CloseBrace,
                    val: String::from("}"),
                    linenum: 1
                },
                TokenType {
                    token: Token::Dollar,
                    val: String::from("$"),
                    linenum: 1
                }
            ]
        );
    }

    #[test]
    fn multi_char_ops() {
        assert_eq!(
            tokenize("&&||>>", "").unwrap(),
            vec![
                TokenType {
                    token: Token::And,
                    val: "&&".to_string(),
                    linenum: 1
                },
                TokenType {
                    token: Token::Or,
                    val: String::from("||"),
                    linenum: 1
                },
                TokenType {
                    token: Token::BitwiseRight,
                    val: String::from(">>"),
                    linenum: 1
                }
            ]
        );
    }

    // #[test]
    // fn drop_whitespace() {
    //     assert_eq!(
    //         tokenize("%=\r34\n   ~").unwrap(),
    //         vec![
    //             TokenType{token: Token::AssignMod,
    //             TokenType{token: Token::Literal(Value::Int(34)),
    //             TokenType{token: Token::BitComp
    //         ]
    //     );
    // }

    // #[test]
    // fn basic_identifiers() {
    //     assert_eq!(
    //         tokenize("async fn pub else").unwrap(),
    //         vec![
    //             Token::Keyword(Keyword::Async),
    //             Token::Keyword(Keyword::Func),
    //             Token::Keyword(Keyword::Pub),
    //             Token::Keyword(Keyword::Else)
    //         ]
    //     );
    // }

    // #[test]
    // fn false_positive_tokens() {
    //     match tokenize("👫 🔫") {
    //         Err(TokenizeError::UnknownToken(_)) => {}
    //         _ => panic!(),
    //     }
    // }
}
