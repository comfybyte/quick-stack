use colored::Colorize;

use crate::{errors::QSError, Rulefile};

pub fn rm(numbers: &[usize]) -> Result<(), QSError> {
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
