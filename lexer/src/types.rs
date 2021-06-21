use itertools::Itertools;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct TokenParser<'a> {
    pub tokens: Vec<Token>,
    iter: Peekable<Chars<'a>>,
}

impl<'a> TokenParser<'a> {
    pub fn new(source: &str) -> TokenParser {
        TokenParser {
            tokens: vec![],
            iter: source.chars().peekable(),
        }
    }

    pub fn push(&mut self, token: Token) {
        self.iter.next();
        self.tokens.push(token);
    }

    pub fn push_back(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn p_next(&mut self) -> Option<char> {
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
    Void,
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParserDescriptor {
    AnyIdentifier,
    AnyVariable,
    Token(Token),
    Newline,
    NoToken,
}
