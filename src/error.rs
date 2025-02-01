use thiserror::Error;

#[derive(Debug, Error)]
pub enum BloomFilterError {
    #[error("Invalid format")]
    InvalidFormat,
}
