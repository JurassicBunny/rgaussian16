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
}

impl GaussError {
    // return the message to be displayed on failure.
    pub(crate) fn new(kind: GaussErrorKind) -> GaussError {
        GaussError { kind }
    }

    fn display(&self) -> &str {
        match self.kind {
            GaussErrorKind::ParaseError => "failed to parse configuration file",
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

// Config errors
#[derive(Debug, Clone)]
pub struct ConfigError {
    kind: ConfigErrorKind,
}

#[derive(Debug, Clone)]
pub(crate) enum ConfigErrorKind {
    Memory,
    CPU,
    GPU,
    KeyWords,
    SeredYaml(String),
}

impl ConfigError {
    pub(crate) fn new(kind: ConfigErrorKind) -> ConfigError {
        ConfigError { kind }
    }

    fn display(&self) -> &str {
        match &self.kind {
            ConfigErrorKind::Memory => "invalid mem variable: try `mem: 100GB, 100MB ... etc.`",
            ConfigErrorKind::CPU => "invalid cpu variable: try `cpu: 0-24 0-39 ... etc.`",
            ConfigErrorKind::GPU => {
                "invalid gpu variable: try `gpu: 0=0 1,2=0,3 1-10=0-9 ... etc.`"
            }
            ConfigErrorKind::KeyWords => "invalid key_word variable: must contain #p #n #b ...etc.",
            ConfigErrorKind::SeredYaml(e) => e.as_str(),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

// Implement source and backtrace for ConfigError
impl Error for ConfigError {}

impl From<serde_yaml::Error> for ConfigError {
    fn from(e: serde_yaml::Error) -> Self {
        ConfigError {
            kind: ConfigErrorKind::SeredYaml(e.to_string()),
        }
    }
}
