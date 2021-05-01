mod cli;
mod config;
mod domain;
mod operations;

use std::convert::TryFrom;
use thiserror::Error as ThisError;

fn main() -> Result<(), Error> {
    let matches = cli::app().get_matches();
    let config = config::Config::try_from(&matches)?;
    operations::actual::run(config)?;

    println!("Done!");

    Ok(())
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    CliArgument(#[from] config::Error),
    #[error(transparent)]
    Actual(#[from] operations::actual::Error),
}
