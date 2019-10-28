use reqwest;
use serde_json;
use std::env::VarError;
use std::io;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    Infallible(std::convert::Infallible),
    VarError(VarError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJsonError(err)
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
