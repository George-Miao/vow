use std::io;

#[cfg(feature = "format-json")]
use serde_json::error::Category;
use thiserror::Error;

/// Errors that can occur when working with [`Vow`] and [`VowAsync`].
///
/// [`Vow`]: super::Vow
/// [`VowAsync`]: super::VowAsync
#[derive(Debug, Error)]
pub enum Error {
    /// Io error
    #[error("Io error: {0}")]
    Io(#[from] io::Error),

    /// Serde json error
    #[cfg(feature = "format-json")]
    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),
}

impl Error {
    /// Check if the error is caused by invalid data (e.g. bad syntax, unexpected EOF, etc.)
    #[must_use]
    pub fn is_invalid_data(&self) -> bool {
        match self {
            #[cfg(feature = "format-json")]
            Self::Json(err) => matches!(
                err.classify(),
                Category::Data | Category::Eof | Category::Syntax
            ),
            _ => false,
        }
    }
}

/// Result type for vow operations.
pub type VowResult<T, E = Error> = Result<T, E>;
