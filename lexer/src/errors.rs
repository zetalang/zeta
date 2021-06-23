use thiserror::Error;

use crate::{ParserDescriptor, Token, TokenType, Type};

#[derive(Error, Debug)]
pub enum TokenizeError {
    #[error("invalid token {0}")]
    UnknownToken(char),
    #[error("invalid integer")]
    InvalidInteger(#[from] std::num::ParseIntError),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unassigned variable")]
    UnassignedVariable,
    #[error("expected {expected:?} got {received:?}")]
    UnexpectedToken {
        expected: ParserDescriptor,
        received: Token,
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
