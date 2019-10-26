use reqwest;
use serde_json;
use std::io;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
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
