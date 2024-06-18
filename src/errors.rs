use clap::parser::MatchesError;
use thiserror::Error;

use crate::rules::RuleField;

#[derive(Error, Debug)]
pub enum QSError {
    #[error("{0}")]
    Rulefile(RulefileError),
    #[error("cannot parse given argument '{0}': {1}")]
    ParseFailed(String, MatchesError),
    #[error("the given expression couldn't be parsed as regex.")]
    InvalidRegex,
    #[error("$EDITOR must be set.")]
    EditorUnset,
    #[error("cannot open rule file for editing: {0:?}")]
    EditFailed(std::io::Error),
    #[error("an unexpected error has occurred: {0}")]
    Other(String),
}

impl From<RulefileError> for QSError {
    fn from(value: RulefileError) -> Self {
        Self::Rulefile(value)
    }
}

#[derive(Error, Debug)]
pub enum RulefileError {
    #[error("cannot read rule file's contents: {0}.")]
    Read(std::io::Error),
    #[error("cannot write rule file changes: {0}.")]
    Write(std::io::Error),
    #[error("cannot check rule file path: {0}.")]
    Check(std::io::Error),
    #[error("cannot find XDG data directory. check if $XDG_DATA_HOME is set.")]
    Find,
    #[error("cannot parse rule #{1}, it's missing field: {0:?}. likely from a malformatted edit.")]
    Parse(RuleField, usize),
    #[error("cannot parse file contents into UTF-8 string.")]
    UTF8Parse,
}
