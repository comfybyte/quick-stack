use clap::Arg;

#[must_use]
pub fn matching() -> Arg {
    Arg::new("matching")
        .long("matching")
        .short('m')
        .value_name("PATTERN")
        .help("A file name pattern to match this rule.")
}

#[must_use]
pub fn from() -> Arg {
    Arg::new("from")
        .long("from")
        .short('f')
        .value_name("PATH")
        .help("Where to read files from.")
}

#[must_use]
pub fn to() -> Arg {
    Arg::new("to")
        .long("to")
        .short('t')
        .value_name("PATH")
        .help("Where to place files after quick stacking.")
}
