use std::{io, str::Utf8Error};
use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serial error: {0}")]
    Serial(#[from] serialport::Error),
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to parse bytes as UTF-8: {0}")]
    Utf8(#[from] Utf8Error),
    #[error("Unexpected command result, expected {expected}, got {got}")]
    UnexpectedResultType {
        expected: &'static str,
        got: &'static str,
    },
    #[error("Received empty result")]
    EmptyResult,
}
