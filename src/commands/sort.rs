use colored::Colorize;
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use crate::Rulefile;

pub fn sort() -> anyhow::Result<()> {
    let mut targets = HashMap::<PathBuf, Vec<(String, PathBuf)>>::new();
    Rulefile::load()?.rules.into_iter().for_each(|rule| {
        let new = (rule.matching, rule.output);

        targets
            .entry(rule.input)
            .and_modify(|target| target.push(new.clone()))
            .or_insert_with(|| Vec::from([new]));
    });

    let start = std::time::Instant::now();

    targets
        .into_iter()
        .filter_map(|(input, targets)| match fs::read_dir(input) {
            Ok(files) => Some((files.flatten().collect::<Vec<_>>(), targets)),
            Err(err) => {
                eprintln!("skipping rules reading from unaccessible directory:\n{err:?}");
                None
            }
        })
        .for_each(|(input_files, targets)| {
            for (matching, destination) in &targets {
                let Ok(matching) = regex::Regex::new(matching) else {
                    eprintln!("skipping rule: `matching` can't be parsed into valid regex.\nmatching: {matching}");
                    continue
                };

                for file in &input_files {
                    if let Some(filename) = file.file_name().to_str() {
                        if !matching.is_match(filename) {
                            continue
                        }
                    } else {
                        eprintln!("can't read filename, probaby not a valid UTF-8.");
                        continue
                    }

                    let from = file.path();
                    let to = {
                        let mut new = destination.clone();
                        new.push(file.file_name());
                        new
                    };

                    stack(&from, &to, destination, true);
                }
            }
        });

    println!("\ndone (took {}ms).", start.elapsed().as_millis());
    Ok(())
}

fn stack(from: &Path, to: &Path, destination: &Path, retry: bool) {
    if let Err(err) = fs::rename(from, to) {
        match err.kind() {
            io::ErrorKind::NotFound => {
                println!(
                    "{} destination, creating it...",
                    " missing ".on_bright_yellow().black()
                );

                if !retry {
                    return;
                }

                if let Err(err) = fs::create_dir(destination) {
                    eprintln!("can't create output directory: {err:?}");
                } else {
                    println!(
                        "{} '{}'.",
                        " created ".on_green().black(),
                        destination.display()
                    );

                    stack(from, to, destination, false);
                }
            }
            kind => eprintln!("couldn't stack file: {kind:?}.\nfile: {from:?}\ntarget: {to:?}"),
        }
    } else {
        println!(
            "{} {} {} {}",
            " stacked ".on_bright_blue().black(),
            from.display(),
            "--->".bright_blue(),
            to.display()
        );
    }
}
