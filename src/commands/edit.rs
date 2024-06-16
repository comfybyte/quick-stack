use colored::Colorize;
use std::{io::stdout, process::Command};

use crate::{errors::QSError, Rulefile};

pub fn edit() -> Result<(), QSError> {
    let editor = std::env::var("EDITOR").map_err(|_| QSError::EditorUnset)?;
    let path = Rulefile::default_path()?;
    let path = path.to_str().ok_or(QSError::Other(
        "couldn't convert rule file path to string.".to_string(),
    ))?;

    Command::new("sh")
        .arg("-c")
        .arg(format!("{editor} {path}"))
        .stdout(stdout())
        .output()
        .map_err(QSError::EditFailed)?;

    println!("{}", "editing done.".bright_blue());
    Ok(())
}
