use clap::{crate_authors, crate_version, Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("home")
                .long("home")
                .takes_value(true)
                .help("Work on the basis that the home directory is at this path")
                .env("HOME"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .takes_value(true)
                .default_value("ellipsis.yml")
                .help("The configuration file for the operations to perform")
                .env("ELLIPSIS"),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .takes_value(false)
                .help("Print what would be done without making any changes"),
        )
}
