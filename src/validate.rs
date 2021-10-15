use crate::error::{ConfigError, ConfigErrorKind};
use crate::gaussian::GaussConfig;

use regex::{Regex, RegexSet};

// Structure to house the config validation requirements
pub(crate) struct Validator {}

// After generating the gaussian config from serde_yaml,
// validate the strings obtained for mem, cpu, gpu and
// key words.
impl Validator {
    pub(crate) fn validate_config(config: &GaussConfig) -> Result<(), ConfigError> {
        Self::validate_mem(config)?;
        Self::validate_cpu(config)?;
        Self::validate_key_words(config)?;

        // Validate the gpu string only if one was provided
        if config.gpu.is_some() {
            Self::validate_gpu(config)?;
        };

        Ok(())
    }

    // Config mem must take the form of some amount of digits,
    // followed by either kb mb gb or tb
    fn validate_mem(config: &GaussConfig) -> Result<(), ConfigError> {
        let mem_regex = Regex::new(r"\d(?i)kb|mb|gb|tb").unwrap();
        let error = ConfigError::new(ConfigErrorKind::Memory);
        let to_match = config.mem.to_string();

        Self::validate(to_match, mem_regex, error)
    }

    // Config cpu must take the form of a digit followed by a dash,
    // and finally another digit
    fn validate_cpu(config: &GaussConfig) -> Result<(), ConfigError> {
        let cpu_regex = Regex::new(r"^[0-9]-[0-9]").unwrap();
        let error = ConfigError::new(ConfigErrorKind::CPU);
        let to_match = config.cpu.to_string();

        Self::validate(to_match, cpu_regex, error)
    }

    // As there are far too many key word combinations in Gaussian16,
    // check only for a `#` followed by a single letter. Allow g16 to
    // fail if invalid key_words are provided.
    //
    // Note: If a key word is required for your program, delegate the
    // validation of said key word to the caller (ie. do not edit this).
    fn validate_key_words(config: &GaussConfig) -> Result<(), ConfigError> {
        let key_regex = Regex::new(r"#(?i)[A-Z]{1}").unwrap();
        let error = ConfigError::new(ConfigErrorKind::KeyWords);
        let to_match = config.key_words.to_string();

        Self::validate(to_match, key_regex, error)
    }

    // GPU may have several correct regex matches. Therefore, this
    // function iterates over a set of regex and returns an error
    // on zero matches or more than one match.
    fn validate_gpu(config: &GaussConfig) -> Result<(), ConfigError> {
        let set = RegexSet::new(&[
            r"^[0-9],([0-9],)+[0-9]=[0-9],([0-9],)+[0-9]",
            r"d*-\d+=\d*-\d+",
            r"^\d{1}=\d{1}",
        ])
        .unwrap();
        let to_match = config.gpu.as_ref();
        let matches: Vec<_> = set.matches(to_match.unwrap()).into_iter().collect();
        if matches.len() == 0 || matches.len() > 1 {
            return Err(ConfigError::new(ConfigErrorKind::GPU));
        } else {
            Ok(())
        }
    }

    // validation function, return an error if to_match does not match the
    // regex provided by the calling function. Otherwise, this function will
    // return Ok(()).
    fn validate(to_match: String, regex: Regex, error: ConfigError) -> Result<(), ConfigError> {
        if !regex.is_match(&to_match) {
            return Err(error);
        };
        Ok(())
    }
}
