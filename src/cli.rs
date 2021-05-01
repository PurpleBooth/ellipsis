use clap::{crate_authors, crate_version, App, Arg};

pub fn app() -> App<'static> {
    App::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("home")
                .long("home")
                .about("Work on the basis that the home directory is at this path")
                .env("HOME"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .default_value("ellipsis.yml")
                .about("The configuration file for the operations to perform")
                .env("ELLIPSIS"),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .takes_value(false)
                .about("Print what would be done without making any changes"),
        )
}
