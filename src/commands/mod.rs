use anyhow::{anyhow, Result};
use colored::Colorize;
use std::{
    fs,
    io::{self, stdout},
    path::PathBuf,
    process::Command,
};
use tracing::warn;

use crate::{Rule, Rulefile};

pub mod parse;

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

pub fn sort() -> Result<()> {
    use io::ErrorKind;
    use regex::Regex;

    // TODO: Optimise this.
    for rule in Rulefile::load()?.rules {
        let input_paths = match fs::read_dir(&rule.input) {
            Ok(target_files) => target_files
                .flatten()
                .map(|file| file.path())
                .collect::<Vec<_>>(),
            Err(err) => {
                warn!("skipping rule: `input` does not point to a readable directory:\nin rule: {rule:?}\ncause: {err:?}");
                continue;
            }
        };

        let matching = Regex::new(&rule.matching)?;
        for path in input_paths {
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                eprintln!("couldn't properly read input file name, likely not a valid UTF-8.");
                continue;
            };

            if matching.is_match(name) {
                let mut old = rule.input.clone();
                old.push(name);
                let mut new = rule.output.clone();
                new.push(name);

                if let Err(err) = fs::rename(&old, &new) {
                    match err.kind() {
                        ErrorKind::NotFound => {
                            println!("rule output directory not found, creating it...");

                            if let Err(err) = fs::create_dir(&rule.output) {
                                eprintln!("can't create output directory: {err:?}");
                                continue;
                            }

                            println!("{}'{}'", "created".green(), &rule.output.display());

                            if let Err(err) = fs::rename(&old, &new) {
                                eprintln!("can't stack file: {err:?}");
                            }
                        }
                        ErrorKind::NotADirectory => {
                            eprintln!(
                                "can't stack file: {} exists and is not a directory.",
                                rule.output.display()
                            );
                        }
                        _ => {
                            eprintln!("can't stack file: {err:?}");
                        }
                    }
                } else {
                    println!(
                        "{} {} {} {}",
                        "moved".on_bright_blue().black(),
                        old.display(),
                        "--->".bright_blue(),
                        new.display()
                    );
                }
            }
        }
    }

    println!("\ndone.");
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
