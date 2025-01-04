use std::fmt::Display;

use http::StatusCode;
use sha2::{Digest as ShaDigest, Sha256, Sha512};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DigestError {
    #[error("Invalid digest format")]
    InvalidFormat,

    #[error("Invalid digest algorithm")]
    InvalidAlgorithm,

    #[error("Invalid hash length")]
    InvalidHashLength,

    #[error("Invalid hash characters")]
    InvalidHashCharacters,
}

impl DigestError {
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidFormat
            | Self::InvalidAlgorithm
            | Self::InvalidHashLength
            | Self::InvalidHashCharacters => StatusCode::BAD_REQUEST,
        }
    }
}

/// Represents a content digest as defined in the OCI Image Specification.
///
/// <https://github.com/opencontainers/image-spec/blob/v1.1.0/descriptor.md#digests>
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Digest {
    pub algorithm: DigestAlgorithm,
    pub hash: String,
}

impl Digest {
    /// Computes the SHA256 digest for the given bytes.
    #[must_use]
    pub fn sha256(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);

        Self {
            algorithm: DigestAlgorithm::Sha256,
            hash: format!("{:x}", hasher.finalize()),
        }
    }

    /// Computes the SHA512 digest for the given data
    #[must_use]
    pub fn sha512(data: &[u8]) -> Self {
        let mut hasher = Sha512::new();
        hasher.update(data);

        Self {
            algorithm: DigestAlgorithm::Sha512,
            hash: format!("{:x}", hasher.finalize()),
        }
    }
}

impl TryFrom<&str> for Digest {
    type Error = DigestError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (algorithm, hash) = value.split_once(':').ok_or(DigestError::InvalidFormat)?;

        let algorithm = DigestAlgorithm::try_from(algorithm)?;
        algorithm.validate_hash(hash)?;

        Ok(Self {
            algorithm,
            hash: hash.to_owned(),
        })
    }
}

impl Display for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.hash)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DigestAlgorithm {
    Sha256,
    Sha512,
}

impl DigestAlgorithm {
    /// Validates the hash string for the given algorithm.
    pub fn validate_hash(&self, hash: &str) -> Result<(), DigestError> {
        let expected_length = match self {
            Self::Sha256 => 64,
            Self::Sha512 => 128,
        };

        if hash.len() != expected_length {
            return Err(DigestError::InvalidHashLength);
        }

        if !hash
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        {
            return Err(DigestError::InvalidHashCharacters);
        }

        Ok(())
    }
}

impl Display for DigestAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Sha256 => "sha256",
                Self::Sha512 => "sha512",
            }
        )
    }
}

impl TryFrom<&str> for DigestAlgorithm {
    type Error = DigestError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "sha256" => Ok(Self::Sha256),
            "sha512" => Ok(Self::Sha512),
            _ => Err(DigestError::InvalidAlgorithm),
        }
    }
}
