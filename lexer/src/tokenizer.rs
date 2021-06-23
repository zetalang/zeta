use crate::{errors::TokenizeError, *};

pub fn tokenize(contents: &str) -> Result<Vec<TokenType>, TokenizeError> {
    let mut tokens = TokenParser::new(contents);
    let mut is_close_single_line = true;
    while let Some(&c) = tokens.peek() {
        if is_close_single_line {
            match c {
                '{' => tokens.push(TokenType {
                    token: Token::OpenBrace,
                    val: String::from("{"),
                }),
                '}' => tokens.push(TokenType {
                    token: Token::CloseBrace,
                    val: String::from("}"),
                }),
                '(' => tokens.push(TokenType {
                    token: Token::OpenParen,
                    val: String::from("("),
                }),
                ')' => tokens.push(TokenType {
                    token: Token::CloseParen,
                    val: String::from(")"),
                }),
                '[' => tokens.push(TokenType {
                    token: Token::OpenSquareParen,
                    val: String::from("["),
                }),
                ']' => tokens.push(TokenType {
                    token: Token::CloseSquareParen,
                    val: String::from("]"),
                }),
                ' ' | '\t' | '\r' | '\n' => tokens.drop(),
                'a'..='z' | 'A'..='Z' => {
                    let word: &str = &tokens.get_string(|x| x.is_ascii() && x.is_alphanumeric());
                    match word {
                        "async" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Async),
                            val: String::from("async"),
                        }),
                        "int" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Int),
                            val: String::from("int"),
                        }),
                        "str" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::String),
                            val: String::from("str"),
                        }),
                        "fn" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Func),
                            val: String::from("fn"),
                        }),
                        "let" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Let),
                            val: String::from("let"),
                        }),
                        "mlstr" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::MLstr),
                            val: String::from("mlstr"),
                        }),
                        "return" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Return),
                            val: String::from("return"),
                        }),
                        "void" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Void),
                            val: String::from("void"),
                        }),
                        "bool" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Bool),
                            val: String::from("bool"),
                        }),
                        "if" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::If),
                            val: String::from("if"),
                        }),
                        "else" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Else),
                            val: String::from("else"),
                        }),
                        "while" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::While),
                            val: String::from("while"),
                        }),
                        "const" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Const),
                            val: String::from("const"),
                        }),
                        "use" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Use),
                            val: String::from("use"),
                        }),
                        "for" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::For),
                            val: String::from("for"),
                        }),
                        "pub" => tokens.push_back(TokenType {
                            token: Token::Keyword(Keyword::Pub),
                            val: String::from("pub"),
                        }),
                        other => tokens.push_back(TokenType {
                            token: Token::Identifier(word.to_string()),
                            val: String::from(other),
                        }),
                    }
                }
                '`' => {
                    tokens.p_next();
                    let t = tokens.get_string(|s| (s != &'`'));
                    tokens.push(TokenType {
                        token: Token::Identifier(t),
                        val: String::from("`"),
                    })
                }
                '"' => {
                    tokens.p_next();
                    let t = tokens.get_string(|s| (s != &'"'));
                    tokens.push(TokenType {
                        token: Token::Identifier(t),
                        val: String::from('"'),
                    })
                }
                '\'' => {
                    tokens.p_next();
                    let t = tokens.get_string(|s| (s != &'\''));
                    tokens.push(TokenType {
                        token: Token::Identifier(t),
                        val: String::from("'"),
                    })
                }
                '0'..='9' => {
                    let word = tokens.get_string(|x| x.is_ascii() && (x.is_digit(16) || x == &'x'));
                    let int: u32 = if word.starts_with("0x") {
                        u32::from_str_radix(&word[2..], 16)?
                    } else {
                        word.parse()?
                    };
                    tokens.push_back(TokenType {
                        token: Token::Literal(Value::Int(int)),
                        val: int.to_string(),
                    })
                }
                '~' => tokens.push(TokenType {
                    token: Token::BitComp,
                    val: String::from('~'),
                }),
                ',' => tokens.push(TokenType {
                    token: Token::Comma,
                    val: String::from(","),
                }),
                multi => match (tokens.p_next().unwrap(), tokens.peek()) {
                    ('&', Some(&'&')) => tokens.push(TokenType {
                        token: Token::And,
                        val: String::from("&&"),
                    }),
                    ('|', Some(&'|')) => tokens.push(TokenType {
                        token: Token::Or,
                        val: String::from("||"),
                    }),

                    ('=', Some(&'>')) => {
                        tokens.p_next();
                        tokens.push(TokenType {
                            token: Token::AsignFunc,
                            val: String::from("=>"),
                        })
                    }
                    ('=', Some(&'=')) => tokens.push(TokenType {
                        token: Token::Equal,
                        val: String::from(","),
                    }),
                    ('<', Some(&'=')) => tokens.push(TokenType {
                        token: Token::LessThanOrEqual,
                        val: String::from("<="),
                    }),
                    ('>', Some(&'=')) => tokens.push(TokenType {
                        token: Token::GreaterThanOrEqual,
                        val: String::from(">="),
                    }),
                    ('!', Some(&'=')) => tokens.push(TokenType {
                        token: Token::NotEqual,
                        val: String::from("!="),
                    }),
                    ('<', Some(&'<')) => {
                        tokens.p_next();
                        if let Some(&'=') = tokens.peek() {
                            tokens.push(TokenType {
                                token: Token::AssignBitLeft,
                                val: String::from("<<="),
                            })
                        } else {
                            tokens.push_back(TokenType {
                                token: Token::BitwiseLeft,
                                val: String::from("<<"),
                            })
                        }
                    }
                    ('>', Some(&'>')) => {
                        tokens.p_next();
                        if let Some(&'=') = tokens.peek() {
                            tokens.push(TokenType {
                                token: Token::AssignBitRight,
                                val: String::from(">>="),
                            })
                        } else {
                            tokens.push_back(TokenType {
                                token: Token::BitwiseRight,
                                val: String::from(">>"),
                            })
                        }
                    }
                    ('+', Some(&'=')) => tokens.push(TokenType {
                        token: Token::AssignAdd,
                        val: String::from("+="),
                    }),
                    ('-', Some(&'=')) => tokens.push(TokenType {
                        token: Token::AssignSub,
                        val: String::from("-="),
                    }),
                    ('*', Some(&'=')) => tokens.push(TokenType {
                        token: Token::AssignMul,
                        val: String::from("*="),
                    }),
                    ('/', Some(&'=')) => tokens.push(TokenType {
                        token: Token::AssignDiv,
                        val: String::from("/="),
                    }),
                    ('/', _) => {
                        let next = tokens.peek().unwrap();
                        if next == &'/' {
                            tokens.p_next();
                            is_close_single_line = false;
                        } else {
                            tokens.push_back(TokenType {
                                token: Token::Division,
                                val: String::from("/"),
                            })
                        }
                    }
                    ('%', Some(&'=')) => tokens.push(TokenType {
                        token: Token::AssignMod,
                        val: String::from("%="),
                    }),
                    ('&', Some(&'=')) => tokens.push(TokenType {
                        token: Token::AssignAnd,
                        val: String::from("&="),
                    }),
                    ('|', Some(&'=')) => tokens.push(TokenType {
                        token: Token::AssignOr,
                        val: String::from("|="),
                    }),
                    ('^', Some(&'=')) => tokens.push(TokenType {
                        token: Token::AssignXor,
                        val: String::from("^="),
                    }),
                    ('+', Some(&'+')) => tokens.push(TokenType {
                        token: Token::Increment,
                        val: String::from("++"),
                    }),
                    ('-', Some(&'-')) => tokens.push(TokenType {
                        token: Token::Decrement,
                        val: String::from("--"),
                    }),
                    (':', Some(&':')) => tokens.push(TokenType {
                        token: Token::DoubleColon,
                        val: String::from("::"),
                    }),

                    ('.', _) => tokens.push_back(TokenType {
                        token: Token::Dot,
                        val: String::from("."),
                    }),
                    ('$', _) => tokens.push_back(TokenType {
                        token: Token::Dollar,
                        val: String::from("$"),
                    }),
                    ('#', _) => tokens.push_back(TokenType {
                        token: Token::HashTag,
                        val: String::from("#"),
                    }),
                    ('<', _) => tokens.push_back(TokenType {
                        token: Token::LessThan,
                        val: String::from("<"),
                    }),
                    ('>', _) => tokens.push_back(TokenType {
                        token: Token::GreaterThan,
                        val: String::from(">"),
                    }),
                    ('!', _) => tokens.push_back(TokenType {
                        token: Token::LogicalNeg,
                        val: String::from("!"),
                    }),
                    ('&', _) => tokens.push_back(TokenType {
                        token: Token::BitwiseAnd,
                        val: String::from("&"),
                    }),
                    ('|', _) => tokens.push_back(TokenType {
                        token: Token::BitwiseOr,
                        val: String::from("|"),
                    }),
                    ('=', _) => tokens.push_back(TokenType {
                        token: Token::Assign,
                        val: String::from("="),
                    }),
                    ('+', _) => tokens.push_back(TokenType {
                        token: Token::Addition,
                        val: String::from("+"),
                    }),
                    ('-', _) => tokens.push_back(TokenType {
                        token: Token::Negation,
                        val: String::from("-"),
                    }),
                    ('*', _) => tokens.push_back(TokenType {
                        token: Token::Multiplication,
                        val: String::from("*"),
                    }),
                    ('%', _) => tokens.push_back(TokenType {
                        token: Token::Modulus,
                        val: String::from("%"),
                    }),
                    ('^', _) => tokens.push_back(TokenType {
                        token: Token::BitwiseXor,
                        val: String::from("^"),
                    }),
                    (':', _) => tokens.push_back(TokenType {
                        token: Token::Colon,
                        val: String::from(":"),
                    }),
                    ('?', _) => tokens.push_back(TokenType {
                        token: Token::Question,
                        val: String::from("?"),
                    }),
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
            vec![
                TokenType {
                    token: Token::OpenBrace,
                    val: String::from("{")
                },
                TokenType {
                    token: Token::CloseBrace,
                    val: String::from("}")
                },
                TokenType {
                    token: Token::Dollar,
                    val: String::from("$")
                }
            ]
        );
    }

    #[test]
    fn multi_char_ops() {
        assert_eq!(
            tokenize("&&||>>").unwrap(),
            vec![
                TokenType {
                    token: Token::And,
                    val: "&&".to_string()
                },
                TokenType {
                    token: Token::Or,
                    val: String::from("||")
                },
                TokenType {
                    token: Token::BitwiseRight,
                    val: String::from(">>")
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
