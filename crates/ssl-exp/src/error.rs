use openssl::error::ErrorStack;
use std::io;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Certificate not found")]
    NoCert,
    #[error("HandshakeError: {0}")]
    Handshake(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Ssl(#[from] ErrorStack),
}