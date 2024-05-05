use clap::Arg;

#[must_use]
pub fn matching() -> Arg {
    Arg::new("matching")
        .long("matching")
        .short('m')
        .value_name("PATTERN")
        .help("A regular expression to match files against. (without /'s)")
}

#[must_use]
pub fn from() -> Arg {
    Arg::new("from")
        .long("from")
        .short('f')
        .value_name("PATH")
        .help("A directory to read files from.")
}

#[must_use]
pub fn to() -> Arg {
    Arg::new("to")
        .long("to")
        .short('t')
        .value_name("PATH")
        .help("A directory to place matching files at.")
}

#[must_use]
pub fn numbers() -> Arg {
    Arg::new("numbers")
        .long("numbers")
        .short('n')
        .value_name("NUMBERS")
        .num_args(1..)
        .help("A list of rule numbers (as listed by the ls subcommand).")
}
