use anyhow::{anyhow, Result};
use std::{fs, io::stdout, path::PathBuf, process::Command};
use tracing::{error, warn};

use crate::{Rule, Rulefile};

pub mod parse;

pub fn add(matching: String, from: PathBuf, to: PathBuf) -> Result<()> {
    use colored::Colorize;

    let rule = Rule { matching, from, to };
    println!(
        "adding rule:\n{} {}\n  {} {} {} {}",
        "for".bright_blue(),
        rule.matching,
        "do".blue(),
        rule.from.display(),
        "-->".blue(),
        rule.to.display()
    );

    let mut rulefile = Rulefile::load()?;
    rulefile.rules.push(rule);
    rulefile.save()?;

    let last_num = rulefile.rules.len() + 1;
    println!("\nrule added as #{}.", last_num.to_string().blue());
    Ok(())
}

pub fn sort() -> Result<()> {
    use regex::Regex;

    // TODO: Optimise this.
    for rule in Rulefile::load()?.rules {
        let mut target_files = match fs::read_dir(&rule.from) {
            Ok(target_files) => target_files,
            Err(err) => {
                warn!("skipping rule: `from` does not point to a readable directory:\nin rule: {rule:?}\ncause: {err:?}");
                continue;
            }
        };

        let matching = Regex::new(&rule.matching)?;
        while let Some(Ok(file)) = target_files.next() {
            let name = match file.file_name().into_string() {
                Ok(name) => name,
                Err(err) => {
                    warn!("failed to read filename: {err:?}");
                    continue;
                }
            };

            if matching.is_match(&name) {
                let mut old = rule.from.clone();
                old.push(&name);
                let mut new = rule.to.clone();
                new.push(&name);

                if let Err(err) = fs::rename(&old, &new) {
                    error!("couldn't move {old:?} -> {new:?}: {err:?}");
                    continue;
                }
            }
        }
    }

    Ok(())
}

pub fn clear() -> Result<()> {
    let mut rulefile = Rulefile::load()?;
    rulefile.rules.clear();
    rulefile.save()?;

    println!("Cleared all rules.");
    Ok(())
}

pub fn ls() -> Result<()> {
    use colored::Colorize;

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
                rule.from.display(),
                "-->".blue(),
                rule.to.display()
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
    use colored::Colorize;

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
    use colored::Colorize;

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
