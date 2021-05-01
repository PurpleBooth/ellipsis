use thiserror::Error as ThisError;

use crate::config::Config;
use crate::domain::Operation;
use crate::operations::{driver, Driver};

pub fn run<T>(input: Config, driver: T) -> Result<T, Error>
where
    T: Driver,
{
    let mut driver = driver;

    for operation in input.operations {
        match operation {
            Operation::Copy { from, to } => driver = driver.copy(&from.location, &to.location)?,
            Operation::Link { from, to } => driver = driver.link(&from.location, &to.location)?,
        };
    }

    Ok(driver)
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Driver(#[from] driver::Error),
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::domain;
    use crate::domain::OperationPath;
    use crate::operations::runner::run;
    use crate::operations::{BlackholeDriver, DriverTypes};

    #[test]
    fn copy_file() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        let input = Config {
            driver: DriverTypes::Blackhole,
            operations: vec![domain::Operation::Copy {
                to: OperationPath::new(
                    &working_dir,
                    &tempfile::tempdir().unwrap().into_path(),
                    "out.txt",
                ),
                from: OperationPath::new(
                    &working_dir,
                    &tempfile::tempdir().unwrap().into_path(),
                    "in.txt",
                ),
            }],
        };

        let driver = run(input, BlackholeDriver::new()).unwrap();

        assert_eq!(
            vec![(
                String::from("copy"),
                format!(
                    "{:?} -> {:?}",
                    working_dir.join("in.txt"),
                    working_dir.join("out.txt")
                )
            )],
            driver.log
        );
    }

    #[test]
    fn link_file() {
        let working_dir = tempfile::tempdir().unwrap().into_path();

        let input = Config {
            driver: DriverTypes::Blackhole,
            operations: vec![domain::Operation::Link {
                to: OperationPath::new(
                    &working_dir,
                    &tempfile::tempdir().unwrap().into_path(),
                    "out.txt",
                ),
                from: OperationPath::new(
                    &working_dir,
                    &tempfile::tempdir().unwrap().into_path(),
                    "in.txt",
                ),
            }],
        };

        let driver = run(input, BlackholeDriver::new()).unwrap();

        assert_eq!(
            vec![(
                String::from("link"),
                format!(
                    "{:?} -> {:?}",
                    working_dir.join("in.txt"),
                    working_dir.join("out.txt")
                )
            )],
            driver.log
        );
    }
}
