// use std::{process::exit, sync::Arc};

// use no_comment::{languages, IntoWithoutComments as _};
// use regex_macro::regex;

// use crate::utils::App;

// pub use self::Token::{
//     Async, Binary, ClosingParenthesis, Comma, Delimiter, Else, For, Ident, If, In, Number,
//     OpeningParenthesis, Operator, Then, Unary, Var,
// };

// #[derive(PartialEq, Clone, Debug)]
// pub enum Token {
//     If,
//     Then,
//     Async,
//     Else,
//     For,
//     In,
//     Binary,
//     Unary,
//     Var,
//     Delimiter,
//     OpeningParenthesis,
//     ClosingParenthesis,
//     Comma,
//     Ident(String),
//     Number(f64),
//     Operator(String),
// }

// pub fn tokenize(input: &str, app: Arc<App>) -> Vec<Token> {
//     let comment_re = regex!(r"(?m)#.*\n");
//     let preprocessed = input
//         .chars()
//         .without_comments(languages::rust())
//         .collect::<String>();

//     let mut result = Vec::new();
//     let token_re = regex!(concat!(
//         r"(?P<ident>\p{Alphabetic}\w*)|",
//         r"(?P<number>\d+\.?\d*)|",
//         r"(?P<delimiter>;)|",
//         r"(?P<oppar>\()|",
//         r"(?P<clpar>\))|",
//         r"(?P<async>\async)",
//         r"(?P<comma>,)|",
//         r"(?P<operator>\S)"
//     ));

//     for cap in token_re.captures_iter(preprocessed.as_str()) {
//         let token = if cap.name("ident").is_some() {
//             match cap.name("ident").unwrap().as_str() {
//                 "if" => If,
//                 "then" => Then,
//                 "else" => Else,
//                 "for" => For,
//                 "in" => In,
//                 "binary" => Binary,
//                 "async" => Async,
//                 "unary" => Unary,
//                 "var" => Var,
//                 ident => Ident(ident.to_string()),
//             }
//         } else if cap.name("number").is_some() {
//             match cap.name("number") {
//                 Some(num) => Number(num.as_str().parse::<f64>().unwrap()),
//                 None => app.error("Lexer: Not a valid number"),
//             }
//         //     Number(1.0)
//         } else if cap.name("delimiter").is_some() {
//             Delimiter
//         } else if cap.name("oppar").is_some() {
//             OpeningParenthesis
//         } else if cap.name("clpar").is_some() {
//             ClosingParenthesis
//         } else if cap.name("comma").is_some() {
//             Comma
//         } else {
//             Operator(cap.name("operator").unwrap().as_str().to_string())
//         };

//         result.push(token)
//     }

//     result
// }
use itertools::Itertools;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct TokenParser<'a> {
    pub tokens: Vec<TokenType>,
    iter: Peekable<Chars<'a>>,
}

impl<'a> TokenParser<'a> {
    pub fn new(source: &str) -> TokenParser {
        TokenParser {
            tokens: vec![],
            iter: source.chars().peekable(),
        }
    }

    pub fn push(&mut self, token: Token, value: &str) {
        self.iter.next();
        self.tokens.push(TokenType {
            token: token,
            val: value.to_string(),
        });
    }

    pub fn push_back(&mut self, token: Token, value: &str) {
        self.tokens.push(TokenType {
            token: token,
            val: value.to_string(),
        });
    }

    pub fn next(&mut self) -> Option<char> {
        self.iter.next()
    }

    pub fn drop(&mut self) {
        self.iter.next();
    }
    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }
    pub fn get_string<F>(&mut self, func: F) -> String
    where
        F: Fn(&char) -> bool,
    {
        self.iter.peeking_take_while(|c| func(c)).collect()
    }
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TokenType {
    token: Token,
    val: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Keyword {
    Int,
    Async,
    String,
    MLstr,
    Let,
    Func,
    Bool,
    Return,
    If,
    Else,
    While,
    Use,
    Pub,
    Const,
    For,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Int(u32),
    Char(String),
    MLStr(u64),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenSquareParen,
    CloseSquareParen,
    Keyword(Keyword),
    Identifier(String),
    Literal(Value),
    BitComp,
    LogicalNeg,
    Negation,
    Addition,
    Multiplication,
    Division,
    Modulus,
    Dot,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    BitwiseLeft,
    BitwiseRight,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    Assign,
    Comma,
    AsignFunc,
    AssignAdd,
    AssignSub,
    AssignDiv,
    AssignMul,
    AssignMod,
    AssignBitLeft,
    AssignBitRight,
    AssignAnd,
    AssignOr,
    AssignXor,
    HashTag,
    Increment,
    Decrement,
    Colon,
    DoubleColon,
    Dollar,
    Question,
}
