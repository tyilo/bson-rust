use std::{io, error, fmt};
use byteorder;

/// Possible errors that can arise during encoding.
#[derive(Debug)]
pub enum EncoderError {
    IoError(io::Error),
}

impl From<io::Error> for EncoderError {
    fn from(err: io::Error) -> EncoderError {
        EncoderError::IoError(err)
    }
}

impl From<byteorder::Error> for EncoderError {
    fn from(err: byteorder::Error) -> EncoderError {
        EncoderError::IoError(From::from(err))
    }
}

impl fmt::Display for EncoderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &EncoderError::IoError(ref inner) => inner.fmt(fmt)
        }
    }
}

impl error::Error for EncoderError {
    fn description(&self) -> &str {
        match self {
            &EncoderError::IoError(ref inner) => inner.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match self {
            &EncoderError::IoError(ref inner) => Some(inner)
        }
    }
}

/// Alias for `Result<T, EncoderError>`.
pub type EncoderResult<T> = Result<T, EncoderError>;
