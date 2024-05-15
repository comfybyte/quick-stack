use crate::{Rule, Rulefile};
use anyhow::{anyhow, Result};
use colored::Colorize;
use std::{io::stdout, path::PathBuf, process::Command};

pub mod parse;
pub mod sort;

pub use sort::*;

pub fn add(matching: String, input: PathBuf, output: PathBuf) -> Result<()> {
    let rule = Rule {
        matching,
        input,
        output,
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
    rulefile.save()?;

    let last_num = rulefile.rules.len();
    println!("\nrule added as #{}.", last_num.to_string().blue());
    Ok(())
}

pub fn clear() -> Result<()> {
    let mut rulefile = Rulefile::load()?;
    rulefile.rules.clear();
    rulefile.save()?;

    println!("cleared all rules.");
    Ok(())
}

pub fn ls() -> Result<()> {
    let rulefile = Rulefile::load()?;

    if rulefile.rules.is_empty() {
        println!("there are no rules defined.");
    } else {
        println!(
            "there are currently {} rules defined:",
            rulefile.rules.len().to_string().bold().blue()
        );
        rulefile.rules.iter().enumerate().for_each(|(i, rule)| {
            println!(
                "[{}] {} {}\n    {} {} {} {}",
                i + 1,
                "for".bright_blue(),
                rule.matching,
                "do".blue(),
                rule.input.display(),
                "-->".blue(),
                rule.output.display()
            );
        });
        println!(
            "\nuse {} to remove rules or {} to manually edit the rule file.",
            "rm".red(),
            "edit".red()
        );
    }

    Ok(())
}

pub fn edit() -> Result<()> {
    let Ok(editor) = std::env::var("EDITOR") else {
        eprintln!("{} requires $EDITOR to be set.", "edit".bright_red());
        return Ok(());
    };
    let path = Rulefile::default_path()?;
    let Some(path) = path.to_str() else {
        return Err(anyhow!("failed to convert rulefile path to &str."));
    };

    if let Err(err) = Command::new("sh")
        .arg("-c")
        .arg(format!("{editor} {path}"))
        .stdout(stdout())
        .output()
    {
        eprintln!("couldn't open file for edit with $EDITOR: {err}",);
    }

    println!("{}", "editing done.".bright_blue());
    Ok(())
}

pub fn rm(numbers: &[usize]) -> Result<()> {
    let mut rulefile = Rulefile::load()?;
    rulefile.rules = rulefile
        .rules
        .iter()
        .enumerate()
        .filter_map(|(i, rule)| {
            let pos = i + 1;
            if numbers.contains(&pos) {
                println!("removed rule #{}.", pos.to_string().bright_red());
                None
            } else {
                Some(rule)
            }
        })
        .cloned()
        .collect();

    rulefile.save()?;
    println!("done.");
    Ok(())
}
