#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::missing_errors_doc)]
#![feature(io_error_more)]

use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

pub mod args;
pub mod commands;
pub mod errors;

use crate::errors::RulefileError;

#[derive(Debug, Clone)]
pub struct Rule {
    matching: String,
    input: PathBuf,
    output: PathBuf,
}

impl Rule {
    fn new(matching: String, input: PathBuf, output: PathBuf) -> Option<Self> {
        if regex_builds(&matching) {
            Some(Self {
                matching,
                input,
                output,
            })
        } else {
            None
        }
    }
}

impl From<&Rule> for String {
    fn from(value: &Rule) -> Self {
        format!(
            "{}\n{}\n{}",
            value.matching,
            value.input.display(),
            value.output.display()
        )
    }
}

#[derive(Debug)]
pub enum RuleField {
    Matching,
    Input,
    Output,
}

#[derive(Debug, Default, Clone)]
pub struct Rulefile {
    rules: Vec<Rule>,
}
impl Rulefile {
    /// Returns the path to the rulefile.
    pub fn default_path() -> Result<PathBuf, RulefileError> {
        xdg::BaseDirectories::with_prefix("quick-stack")
            .map_err(|_| RulefileError::Find)?
            .place_data_file("rulefile")
            .map_err(RulefileError::Check)
    }

    /// Returns a manipulatable instance of the rulefile.
    pub fn load() -> Result<Self, RulefileError> {
        Self::read_as_string()?.try_into()
    }

    /// Reads the rule file's contents as a string.
    pub fn read_as_string() -> Result<String, RulefileError> {
        let bytes = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(Self::default_path()?)
            .map_err(RulefileError::Read)?
            .bytes()
            .flatten()
            .collect::<Vec<u8>>();

        String::from_utf8(bytes).map_err(|_| RulefileError::UTF8Parse)
    }

    /// Writes to disk any changes made to this instance.
    pub fn save(&self) -> Result<(), RulefileError> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Self::default_path()?)
            .map_err(RulefileError::Read)?
            .write_all(String::from(self).as_bytes())
            .map_err(RulefileError::Write)?;

        Ok(())
    }
}

impl TryFrom<String> for Rulefile {
    type Error = RulefileError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(Self::default())
        } else {
            let chunks = value
                .split("\n\n")
                .map(|chunk| chunk.lines())
                .collect::<Vec<_>>();
            let mut rules = Vec::with_capacity(chunks.len());

            for (num, mut chunk) in chunks.into_iter().enumerate() {
                let Some(matching) = chunk.next() else {
                    return Err(RulefileError::Parse(RuleField::Matching, num));
                };
                let Some(input) = chunk.next() else {
                    return Err(RulefileError::Parse(RuleField::Input, num));
                };
                let Some(output) = chunk.next() else {
                    return Err(RulefileError::Parse(RuleField::Output, num));
                };

                rules.push(Rule {
                    matching: matching.into(),
                    input: input.into(),
                    output: output.into(),
                });
            }

            Ok(Self { rules })
        }
    }
}

impl From<&Rulefile> for String {
    fn from(value: &Rulefile) -> Self {
        value
            .rules
            .iter()
            .map(Self::from)
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

fn regex_builds(value: &str) -> bool {
    regex::Regex::new(value).is_ok()
}
