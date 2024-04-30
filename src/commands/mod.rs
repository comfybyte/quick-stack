use anyhow::Result;
use std::path::PathBuf;

use crate::{Rule, Rulefile};

pub mod parse;

/// Append item to rule file.
pub fn add(matching: String, from: PathBuf, to: PathBuf) -> Result<()> {
    Rulefile::new()?.push(Rule { matching, from, to }).save()?;

    Ok(())
}
