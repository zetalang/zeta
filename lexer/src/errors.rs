use thiserror::Error;

use crate::{ParserDescriptor, Token, Type};

#[derive(Error, Debug)]
pub enum TokenizeError {
    #[error("invalid token {c:?}, in {filename:}:{linenum:?}")]
    UnknownToken{
        c: char,
        linenum: i32,
        filename: Box<str>
    },
    #[error("invalid integer ")]
    InvalidInteger(#[from] std::num::ParseIntError),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unassigned variable, in {filename:}:{linenum:?}")]
    UnassignedVariable{
        linenum: i32,
        filename: Box<str>,
    },
    #[error("expected {expected:?} got {received:?}: in {filename:}:{linenum:?}")]
    UnexpectedToken {
        expected: ParserDescriptor,
        received: Token,
        linenum: i32,
        filename: Box<str>
    },
    #[error("expected {expected:?} got {received:?} {filename:}:{linenum:?}\n\tIn function {fnname:}")]
    UnexpectedType { 
        expected: Box<str>,
        received: Token, 
        fnname: Box<str>, 
        linenum: i32,
        filename: Box<str>
    },
    #[error("{msg:}")]
    EndOfLine {
        msg: Box<str>
    },
    #[error("expected {expected:?} to be present")]
    AbsentToken { expected: ParserDescriptor },
    #[error("expected return type to be present in function: {fnname:?}, File: {filename:}:{linenum:}")]
    AbsentReturnType{
        fnname: Box<str>,
        linenum: i32,
        filename: Box<str>
    },
    #[error("error tokenizing")]
    TokenizeError(#[from] TokenizeError),
    #[error("An unkown error occured \n\t NOTE: If this continues open an issue in https://github.com/zetacli/zetac/issues")]
    Unknown,
}
