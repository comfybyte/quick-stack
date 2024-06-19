use clap::{crate_version, Command};
use colored::Colorize;
use quick_stack::{args, commands};

fn cli() -> Command {
    Command::new("quick-stack")
        .about("Quickly organise files based on predefined rules.")
        .version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new sorting rule.")
                .arg(args::matching().required(true))
                .arg(args::input().required(true))
                .arg(args::output().required(true))
                .after_help("`matching` may take regular expressions (no wrapping / needed)."),
        )
        .subcommand(Command::new("sort").about("Quickstack files according to sorting rules."))
        .subcommand(Command::new("clear").about("Clear all rules."))
        .subcommand(Command::new("ls").about("List all rules."))
        .subcommand(Command::new("edit").about("Open the file containing all rules for editing."))
        .subcommand(
            Command::new("rm")
                .about("Remove given rules by number.")
                .arg(args::numbers().required(true)),
        )
}

fn main() {
    if cfg!(debug_assertions) {
        better_panic::install();
    }

    if let Err(err) = match cli().get_matches().subcommand() {
        Some(("add", sub_args)) => commands::parse::add(sub_args),
        Some(("sort", _)) => commands::sort(),
        Some(("clear", _)) => commands::clear(),
        Some(("ls", _)) => commands::ls(),
        Some(("edit", _)) => commands::edit(),
        Some(("rm", sub_args)) => commands::parse::rm(sub_args),
        _ => {
            cli().print_help().expect("cannot print help.");
            Ok(())
        }
    } {
        eprintln!("{} {err}", " error ".on_red().black());
    }
}
