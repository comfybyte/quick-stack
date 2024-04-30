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
                .arg(args::to().required(true)),
        )
}

fn main() -> anyhow::Result<()> {
    better_panic::install();

    match cli().get_matches().subcommand() {
        Some(("add", sub_args)) => commands::parse::add(sub_args),
        _ => unreachable!(),
    }?;

    Ok(())
}
