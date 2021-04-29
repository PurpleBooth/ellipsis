mod cli;

use thiserror::Error as ThisError;

fn main() -> Result<(), Error> {
    let matches = cli::app().get_matches();
    matches.value_of("home").ok_or(Error::HomeMissing)?;
    matches.value_of("config").ok_or(Error::ConfigMissing)?;

    println!("Hello, world!");

    Ok(())
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("unable to get home directory from arguments")]
    HomeMissing,
    #[error("unable to get config file from arguments")]
    ConfigMissing,
}
