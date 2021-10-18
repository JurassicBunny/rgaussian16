use crate::error::ConfigError;
use crate::validate::Validator;

use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::process::{Command, Stdio};

use serde::Deserialize;

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

/// Interface for the Gaussian16 quantum chemical package.
///
/// Utilizing a yaml configuration file, generate a Gaussian object.
/// This structure provides functions for auto generating input, and
/// running the external program `g16`.
///
/// As configuration is tied to Gaussian, multiple Gaussian objects may
/// be use to extract different result form Gaussian16. This greatly
/// simplifies the processes of interfacing with the quantum chemical
/// package.
///
/// # Example
///
/// Generate Gaussian object write input and run `g16`
///
/// --------------------------------------------------------
/// ```rust
/// use rgaussian16::Gaussion;
///
/// fn main() {
///     let input_file = std::fs::File::create("input.com").unwrap();
///     let output_file = std::fs::File::create("output.out").unwrap();
///
///     let job1_config = std::fs::File::open("config.yaml").unwrap();
///     let job1_interface = Gaussion::new(config).unwrap();
///
///     job1_interface.gen_input(input_file).unwrap();
///     job1_interface.run(input_file, output_file).unwrap();
/// }
/// ```
/// --------------------------------------------------------
///
///
///
#[derive(Debug, Clone)]
pub struct Gaussian {
    config: GaussConfig,
}

impl Gaussian {
    /// Takes a `.yaml` config file and parses the data. Returns a Gaussian
    /// object with which a user may interface with the Gaussian16 quantum
    /// chemical package.
    pub fn new(config: File) -> Result<Gaussian, ConfigError> {
        let config = Gaussian::parse_config(config)?;
        Ok(Gaussian { config })
    }

    /// Generate input for the Gaussian16 quantum chemical package.
    ///
    /// NOTE: function does not write molecular coordinates. Instead,
    /// it is up to the user to attach coordinates and any other additional
    /// information such as ModRedundant coords to the end of the generated
    /// output.
    pub fn gen_input(self, mut file: File) -> Result<(), std::io::Error> {
        file.write_all(self.to_string().as_bytes())
    }

    /// Run Gaussian16 given the input and output files. As this function requires
    /// the use of raw file descriptors, an unsafe code block is use to convert
    /// between the files provided by the user in the run function parameters into raw
    /// file descriptors. Returns the exit status of Gaussian16 or an io error.
    pub fn run(input: File, output: File) -> Result<std::process::ExitStatus, std::io::Error> {
        let (input, output) = unsafe {
            (
                Stdio::from_raw_fd(input.into_raw_fd()),
                Stdio::from_raw_fd(output.into_raw_fd()),
            )
        };
        Command::new("g16")
            .stdin(input)
            .stdout(output)
            .spawn()
            .unwrap()
            .wait()
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
            Some(gpu) => self.gpu_output(gpu.to_string()),
            None => self.cpu_output(),
        }
    }

    // generates gpu input string
    fn gpu_output(&self, gpu: String) -> String {
        format_args!(
            "%Mem={}\n%Cpu={}\n%Gpu={}\n%Check={}\n{}\n\n {}\n\n{} {}",
            self.config.mem,
            self.config.cpu,
            gpu,
            self.config.checkpoint,
            self.config.key_words,
            self.config.title,
            self.config.charge,
            self.config.multiplicity
        )
        .to_string()
    }

    // generates cpu input string
    fn cpu_output(&self) -> String {
        format_args!(
            "%Mem={}\n%Cpu={}\n%Check={}\n{}\n\n {}\n\n{} {}",
            self.config.mem,
            self.config.cpu,
            self.config.checkpoint,
            self.config.key_words,
            self.config.title,
            self.config.charge,
            self.config.multiplicity
        )
        .to_string()
    }
}

impl Display for Gaussian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}
