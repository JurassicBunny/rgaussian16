use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub(crate) struct GaussError {
    kind: GaussErrorKind,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GaussErrorKind {
    UnKnownError,
}

impl GaussError {
    fn __display(&self) -> &str {
        match self.kind {
            GaussErrorKind::UnKnownError => "an unknown error occurred",
        }
    }
}

impl Display for GaussError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.__display().fmt(f)
    }
}

// Implement source and backtrace for GaussError
impl Error for GaussError {}
