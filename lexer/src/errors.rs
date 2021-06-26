use thiserror::Error;

use crate::{ParserDescriptor, Token, Type};

#[derive(Error, Debug)]
pub enum TokenizeError {
    #[error("invalid token {c:?}, in line {linenum:?}")]
    UnknownToken{
        c: char,
        linenum: i32
    },
    #[error("invalid integer")]
    InvalidInteger(#[from] std::num::ParseIntError),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unassigned variable, in {filename:}:{linenum:?}")]
    UnassignedVariable{
        linenum: i32,
        filename: Box<str>,
    },
    #[error("expected {expected:?} got {received:?}: in line: {linenum:?}")]
    UnexpectedToken {
        expected: ParserDescriptor,
        received: Token,
        linenum: i32,
    },
    #[error("expected {expected:?} got {received:?}")]
    UnexpectedType { expected: Type, received: Type },
    #[error("expected {expected:?} to be present")]
    AbsentToken { expected: ParserDescriptor },
    #[error("expected return type to be present")]
    AbsentReturnType,
    #[error("error tokenizing")]
    TokenizeError(#[from] TokenizeError),
    #[error("unknown error")]
    Unknown,
}
