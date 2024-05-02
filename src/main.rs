use clap::{crate_version, Command};
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
                .arg(args::from().required(true))
                .arg(args::to().required(true))
                .after_help("`matching` may take regular expressions (no wrapping / needed)."),
        )
        .subcommand(Command::new("sort").about("Quickstack files according to sorting rules."))
        .subcommand(Command::new("clear").about("Clear all rules."))
        .subcommand(Command::new("ls").about("List all rules."))
        .subcommand(Command::new("edit").about("Open the file containing all rules for editing."))
}

fn main() -> anyhow::Result<()> {
    better_panic::install();
    tracing_subscriber::fmt::init();

    match cli().get_matches().subcommand() {
        Some(("add", sub_args)) => commands::parse::add(sub_args),
        Some(("sort", _)) => commands::sort(),
        Some(("clear", _)) => commands::clear(),
        Some(("ls", _)) => commands::ls(),
        Some(("edit", _)) => commands::edit(),
        _ => {
            cli().print_help()?;
            Ok(())
        }
    }?;

    Ok(())
}
