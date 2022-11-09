use std::error::Error as StdError;
use std::io::Error as IoError;


/// The main `Result` type for this library.
pub type Result<T> = std::result::Result<T, Error>;

/// The main `Error` enum for this library.
/// It's only wrapping Io Errors for now, but it might be useful later
/// to have a dedicated error enum.
#[derive(Debug)]
pub enum Error {
    /// An [`std::io`] error.
    Io(IoError),
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => std::fmt::Display::fmt(&e, f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Io(e) => Some(e),
        }
    }
}
