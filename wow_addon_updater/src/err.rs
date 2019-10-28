use actix_web::client::SendRequestError;
use actix_web::error::PayloadError;
use serde_json;
use std::env::VarError;
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    SerdeJsonError(serde_json::Error),
    Infallible(std::convert::Infallible),
    VarError(VarError),
    SendRequestError(SendRequestError),
    PayloadError(PayloadError),
    FromUtf8Error(FromUtf8Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJsonError(err)
    }
}

impl From<SendRequestError> for Error {
    fn from(err: SendRequestError) -> Self {
        Self::SendRequestError(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Self::FromUtf8Error(err)
    }
}

impl From<PayloadError> for Error {
    fn from(err: PayloadError) -> Self {
        Self::PayloadError(err)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(err: std::convert::Infallible) -> Self {
        Self::Infallible(err)
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Self::VarError(err)
    }
}
