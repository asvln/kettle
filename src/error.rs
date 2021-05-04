use thiserror::Error;

pub type Result<T> = std::result::Result<T, KettleError>;

/// KettleError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum KettleError {
    /// Represents all other `std::io` errors.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Represents all `ini` parsing Errors;
    #[error(transparent)]
    IniParseError(#[from] ini::ini::ParseError),

    /// Represents all other `ini` Errors;
    #[error(transparent)]
    IniError(#[from] ini::ini::Error),
}
