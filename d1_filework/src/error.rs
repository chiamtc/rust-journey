extern crate failure;

use failure::Error;
use failure_derive::*;
#[derive(Debug, Fail)]
pub enum TransactionError {
    #[fail(display = "invalid file loading error: {}", 0)]
    LoadError(std::io::Error),
    #[fail(display = "parse error: {}", 0)]
    ParseError(serde_json::Error),
    #[fail(display = "message error: {}", 0)]
    Mess(&'static str),
}

//implementing From trait for enum TransactionError so that in match case, we can use e.into() to get the string of our specific errors
impl From<std::io::Error> for TransactionError {
    //from() is a requried function to implement from From trait
    // signature -> from(T) -> Self
    // Self is a reserved word, the type we're working with right now aka TransactionError since the impl is "for TransactionError"
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    //from() is a requried function to implement from From trait
    // signature -> from(T) -> Self
    // Self is a reserved word, the type we're working with right now aka TransactionError since the impl is "for TransactionError"
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Mess(e)
    }
}
