use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),

    PklSend,
    PklRecv,
    PklMalformedResponse { message: String },
    PklProcessStart,
    PklServerError { pkl_error: String },

    SerializeAst,
    DecodeError(String),
    DeserializeError(String),

    MsgpackSerializeError(rmp_serde::encode::Error),
    MsgpackEncodeError(rmpv::encode::Error),
    MsgpackDecodeError(rmpv::decode::Error),

    Eof,
    Syntax,
    ExpectedBoolean,
    ExpectedInteger,
    ExpectedString,
    ExpectedNull,
    ExpectedArray,
    ExpectedArrayComma,
    ExpectedArrayEnd,
    ExpectedMap,
    ExpectedMapColon,
    ExpectedMapComma,
    ExpectedMapEnd,
    ExpectedEnum,
    TrailingCharacters,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg)
            | Error::DeserializeError(msg)
            | Error::DecodeError(msg)
            | Error::PklServerError { pkl_error: msg }
            | Error::PklMalformedResponse { message: msg } => formatter.write_str(msg),

            Error::MsgpackDecodeError(e) => formatter.write_str(&e.to_string()),

            Error::Eof => formatter.write_str("unexpected end of input"),

            _ => formatter.write_str("unknown error"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Message(e.to_string())
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(e: rmp_serde::encode::Error) -> Self {
        Error::MsgpackSerializeError(e)
    }
}

impl From<rmpv::encode::Error> for Error {
    fn from(e: rmpv::encode::Error) -> Self {
        Error::MsgpackEncodeError(e)
    }
}

impl From<rmpv::decode::Error> for Error {
    fn from(e: rmpv::decode::Error) -> Self {
        Error::MsgpackDecodeError(e)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Error::Message(e.to_string())
    }
}
