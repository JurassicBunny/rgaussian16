use crate::error::{GaussError, GaussErrorKind};

use serde::Deserialize;
use std::fmt::Display;
use std::fs::File;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GaussConfig {
    pub(crate) mem: String,
    pub(crate) cpu: String,
    pub(crate) gpu: Option<String>,
    pub(crate) checkpoint: String,
    pub(crate) key_words: String,
    pub(crate) title: String,
    pub(crate) charge: i64,
    pub(crate) multiplicity: u64,
}

#[derive(Debug, Clone)]
pub struct GaussInput {
    config: GaussConfig,
}

impl GaussInput {
    pub fn new(config: File) -> Result<GaussInput, GaussError> {
        let config = GaussInput::parse_config(config)?;
        Ok(GaussInput { config })
    }

    fn parse_config(config: File) -> Result<GaussConfig, GaussError> {
        match serde_yaml::from_reader(config) {
            Ok(config) => Ok(config),
            Err(_) => Err(GaussError {
                kind: GaussErrorKind::ParaseError,
            }),
        }
    }

    fn display(&self) -> String {
        match &self.config.gpu {
            Some(_gpu) => self.gpu_output(),
            None => self.cpu_output(),
        }
    }

    fn gpu_output(&self) -> String {
        let result = format!(
            "%Mem={}\n%Cpu={}\n%Gpu={}\n%Check={}\n#p {}\n\n {}\n\n{} {}",
            self.config.mem,
            self.config.cpu,
            self.config.gpu.as_ref().unwrap(),
            self.config.checkpoint,
            self.config.key_words,
            self.config.title,
            self.config.charge,
            self.config.multiplicity
        );
        result
    }

    fn cpu_output(&self) -> String {
        let result = format!(
            "%Mem={}\n%Cpu={}\n%Check={}\n#p {}\n\n {}\n\n{} {}",
            self.config.mem,
            self.config.cpu,
            self.config.checkpoint,
            self.config.key_words,
            self.config.title,
            self.config.charge,
            self.config.multiplicity
        );
        result
    }
}

impl Display for GaussInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}
