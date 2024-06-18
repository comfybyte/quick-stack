use crate::{errors::QSError, rules::Rulefile};
use colored::Colorize;

pub fn ls() -> Result<(), QSError> {
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
                "[{}] {} {} {} {} {} {}",
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
