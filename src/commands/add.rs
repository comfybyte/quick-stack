use colored::Colorize;
use std::path::PathBuf;

use crate::{
    errors::QSError,
    rules::{Rule, Rulefile},
};

pub fn add(matching: String, input: PathBuf, output: PathBuf) -> Result<(), QSError> {
    let Some(rule) = Rule::new(matching, input, output) else {
        return Err(QSError::InvalidRegex);
    };

    println!(
        "adding rule:\n{} {}\n  {} {} {} {}",
        "for".bright_blue(),
        rule.matching,
        "do".blue(),
        rule.input.display(),
        "-->".blue(),
        rule.output.display()
    );

    let mut rulefile = Rulefile::load()?;
    rulefile.rules.push(rule);
    rulefile.commit()?;

    let last_num = rulefile.rules.len();
    println!("\nrule added as #{}.", last_num.to_string().blue());
    Ok(())
}
