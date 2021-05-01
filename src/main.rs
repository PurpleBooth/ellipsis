use std::convert::TryFrom;

use thiserror::Error as ThisError;

mod cli;
mod config;
mod domain;
mod operations;

fn main() -> Result<(), Error> {
    let matches = cli::app().get_matches();
    let config = config::Config::try_from(&matches)?;

    match config.driver {
        operations::DriverTypes::Io => {
            operations::run(config, operations::IoDriver::new())?;
        }

        operations::DriverTypes::Blackhole => {
            let driver = operations::run(config, operations::BlackholeDriver::new())?;

            for (operation, message) in driver.log {
                println!("{}: {}", operation, message)
            }
        }
    }

    Ok(())
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    CliArgument(#[from] config::Error),
    #[error(transparent)]
    Actual(#[from] operations::RunnerError),
}
