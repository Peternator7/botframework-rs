use serde_json;
use std::error;
use std::convert::From;
use std::str;
use base64;

#[derive(Debug,Eq, PartialEq, Hash, Copy, Clone)]
pub enum CryptoProviderError {
    InvalidKey,
    UnimplementedAlgorithm,
    ValidationError,
    SigningError,
    DefaultError,
    NoAlgorithmSpecified,
}

#[derive(Debug)]
pub enum Error {
    Base64Error(base64::DecodeError),
    SerdeError(serde_json::Error),
    TextError(Box<error::Error>),
    CryptoError(CryptoProviderError),
    MalformedJwt,
    InvalidAlgorithmName,
    SignatureMissing,
}

impl From<strum::ParseError> for Error {
    fn from(_: strum::ParseError) -> Error {
        Error::InvalidAlgorithmName
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::SerdeError(e)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(e: str::Utf8Error) -> Error {
        Error::TextError(Box::new(e))
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Error {
        Error::Base64Error(e)
    }
}

impl From<CryptoProviderError> for Error { 
    fn from(e: CryptoProviderError) -> Error {
        Error::CryptoError(e)
    }
}