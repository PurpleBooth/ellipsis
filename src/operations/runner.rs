use thiserror::Error as ThisError;

use crate::config::Config;
use crate::domain;
use crate::domain::{Driver, Operation};

pub fn run<T>(input: Config, driver: T) -> Result<T, Error>
where
    T: Driver,
{
    input
        .operations
        .into_iter()
        .try_fold(driver, |driver, operation| match operation {
            Operation::Copy { from, to } => driver.copy(&from.location, &to.location),
            Operation::Link {
                from,
                to,
                overwrite,
            } => driver.link(&from.location, &to.location, overwrite),
            Operation::Exec {
                working_dir,
                command,
                args,
            } => driver.exec(&working_dir, &command, &args),
        })
        .map_err(Error::from)
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("driver error")]
    Driver(#[from] domain::Error),
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::domain;
    use crate::domain::{DriverTypes, OperationPath};
    use crate::operations::runner::run;
    use crate::operations::BlackholeDriver;

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
    fn exec_file() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        let input = Config {
            driver: DriverTypes::Blackhole,
            operations: vec![domain::Operation::Exec {
                working_dir: working_dir.clone(),
                command: "bash".into(),
                args: vec!["Hello".into()],
            }],
        };

        let driver = run(input, BlackholeDriver::new()).unwrap();

        assert_eq!(
            vec![(
                String::from("exec"),
                format!("in {:?} \"bash\" [\"Hello\"]", working_dir,)
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
                overwrite: false,
            }],
        };

        let driver = run(input, BlackholeDriver::new()).unwrap();

        assert_eq!(
            vec![(
                String::from("link"),
                format!(
                    "{:?} -> (overwriting: false) {:?}",
                    working_dir.join("in.txt"),
                    working_dir.join("out.txt")
                )
            )],
            driver.log
        );
    }
}
