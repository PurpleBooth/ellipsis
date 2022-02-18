use std::convert::TryFrom;

use thiserror::Error as ThisError;

mod cli;
mod config;
mod domain;
mod operations;
use anyhow::Result as AnyhowResult;

fn main() -> AnyhowResult<()> {
    let matches = cli::cli().get_matches();
    let config = config::Config::try_from(&matches)?;

    match config.driver {
        domain::DriverTypes::Io => {
            operations::run(config, operations::IoDriver::new())?;
        }

        domain::DriverTypes::Blackhole => {
            let driver = operations::run(config, operations::BlackholeDriver::new())?;

            for (operation, message) in driver.log {
                println!("{}: {}", operation, message);
            }
        }
    }

    Ok(())
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("cli argument error")]
    CliArgument(#[from] config::Error),
    #[error("operation runner error")]
    Runner(#[from] operations::RunnerError),
}
