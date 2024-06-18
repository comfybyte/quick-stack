use crate::errors::RulefileError;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Debug, Clone)]
/// Represents a sorting rule.
pub struct Rule {
    pub matching: String,
    pub input: PathBuf,
    pub output: PathBuf,
}

impl Rule {
    #[must_use]
    pub fn new(matching: String, input: PathBuf, output: PathBuf) -> Option<Self> {
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
/// Represents a list of rules created.
pub struct Rulefile {
    pub rules: Vec<Rule>,
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
    pub fn commit(&self) -> Result<(), RulefileError> {
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

    /// Removes rule number `n` from this rule file instance.
    ///
    /// # Errors
    /// If `n == 0` or `n > rules.length`.
    pub fn remove_rule(&mut self, n: usize) -> Result<(), OutOfBoundsError> {
        let Some(idx) = n.checked_sub(1) else {
            return Err(OutOfBoundsError::Zero);
        };

        if self.rules.get(idx).is_some() {
            self.rules.remove(idx);
            Ok(())
        } else {
            Err(OutOfBoundsError::Overflow)
        }
    }
}

pub enum OutOfBoundsError {
    Zero,
    Overflow,
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
