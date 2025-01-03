use std::{error::Error, fmt::Display};
use wayfind_percent::errors::PercentDecodingError;
use wayfind_punycode::errors::PunycodeDecodingError;

/// Errors relating to attempting to decode strings.
#[derive(Debug, PartialEq, Eq)]
pub enum EncodingError {
    Percent(PercentDecodingError),
    Punycode(PunycodeDecodingError),
}

impl Error for EncodingError {}

impl Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Percent(error) => error.fmt(f),
            Self::Punycode(error) => error.fmt(f),
        }
    }
}

impl From<PercentDecodingError> for EncodingError {
    fn from(error: PercentDecodingError) -> Self {
        Self::Percent(error)
    }
}

impl From<PunycodeDecodingError> for EncodingError {
    fn from(error: PunycodeDecodingError) -> Self {
        Self::Punycode(error)
    }
}
