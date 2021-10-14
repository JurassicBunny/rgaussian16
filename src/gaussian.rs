use crate::error::ConfigError;
use crate::validate::Validator;

use serde::Deserialize;
use std::fmt::Display;
use std::fs::File;

// Configuration for Gaussian input file. In serde_yaml, a none value is represented with `~`
// typical input file looks like:
//
// %Mem=134GB
// %Cpu=0-39
// %Checkpoint=test.chk
// #p BP86/Def2svp/W06 SCF=XQC
//
// title card
//
// 0 1
// molecular coords
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
    /// takes a `.yaml` config file and parses the data. Returns a GaussInput with which Gaussian16
    /// input files may be generated using the `to_string()` method implemented via the display
    /// trait.
    pub fn new(config: File) -> Result<GaussInput, ConfigError> {
        let config = GaussInput::parse_config(config)?;
        Ok(GaussInput { config })
    }

    // parse the configuration file and return either a GaussConfig or an Error.
    // Once parsed validate the input for string vars via the Validator.
    fn parse_config(config: File) -> Result<GaussConfig, ConfigError> {
        let config: GaussConfig = serde_yaml::from_reader(config)?;
        Validator::validate_config(&config)?;
        Ok(config)
    }

    // Gaussian16 may be run with gpus. This function checks the config file for Some(gpu) string.
    // If provided, generate gpu input string. Otherwise, return cpu only input.
    fn display(&self) -> String {
        match &self.config.gpu {
            Some(_) => self.gpu_output(),
            None => self.cpu_output(),
        }
    }

    // generates gpu input string
    fn gpu_output(&self) -> String {
        let result = format!(
            "%Mem={}\n%Cpu={}\n%Gpu={}\n%Check={}\n{}\n\n {}\n\n{} {}",
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

    // generates cpu input string
    fn cpu_output(&self) -> String {
        let result = format!(
            "%Mem={}\n%Cpu={}\n%Check={}\n{}\n\n {}\n\n{} {}",
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
