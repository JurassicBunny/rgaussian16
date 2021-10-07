use std::error::Error;
use std::fmt::Display;

/// Generating Gaussian input may fail. GaussError provides the correct error message when input
/// generation fails.
#[derive(Debug, Clone, Copy)]
pub struct GaussError {
    pub(crate) kind: GaussErrorKind,
}

// GaussError may return either a paring error or an unknown error
#[derive(Debug, Clone, Copy)]
pub(crate) enum GaussErrorKind {
    ParaseError,
    UnKnownError,
}

impl GaussError {
    // return the message to be displayed on failure.
    fn display(&self) -> &str {
        match self.kind {
            GaussErrorKind::ParaseError => "failed to parse configuration file",
            GaussErrorKind::UnKnownError => "an unknown error occurred",
        }
    }
}

impl Display for GaussError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

// Implement source and backtrace for GaussError
impl Error for GaussError {}
