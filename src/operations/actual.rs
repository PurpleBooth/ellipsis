use std::fs;
use std::path::PathBuf;

use thiserror::Error as ThisError;

use crate::config::Config;
use crate::domain::Operation;

pub(crate) fn run(input: Config) -> Result<(), Error> {
    for operation in input.operations {
        match operation {
            Operation::Copy { from, to } => fs::copy(from.canonicalize(), to.canonicalize())
                .map_err(|error| {
                    Error::Copy(from.canonicalize(), to.canonicalize(), error.to_string())
                })
                .map(|_| ())?,
        }
    }

    Ok(())
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("copy from `{0}` to `{1}` failed with `{2}`")]
    Copy(PathBuf, PathBuf, String),
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};

    use crate::config::Config;
    use crate::domain;
    use crate::domain::OperationPath;
    use crate::operations::actual::run;

    #[test]
    fn copy_to_working_dir_no_magic() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        let mut file = File::create(&working_dir.join("in.txt")).unwrap();
        write!(file, "Hello, World!").unwrap();

        let input = Config {
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

        run(input).unwrap();

        let mut output_file_contents = String::new();
        File::open(working_dir.join("out.txt"))
            .unwrap()
            .read_to_string(&mut output_file_contents)
            .unwrap();

        assert_eq!(String::from("Hello, World!"), output_file_contents)
    }

    #[test]
    fn copy_to_home() {
        let home = tempfile::tempdir().unwrap().into_path();
        let working_dir = tempfile::tempdir().unwrap().into_path();
        let file_path = &working_dir.join("in.txt");
        let mut file = File::create(file_path).unwrap();
        write!(file, "Hello, World!").unwrap();

        let input = Config {
            operations: vec![domain::Operation::Copy {
                to: OperationPath::new(&working_dir, &home, "~/out.txt"),
                from: OperationPath::new(&working_dir, &home, "in.txt"),
            }],
        };

        run(input).unwrap();

        let mut output_file = File::open(home.join("out.txt")).unwrap();
        let mut output_file_contents = String::new();
        output_file
            .read_to_string(&mut output_file_contents)
            .unwrap();

        assert_eq!(String::from("Hello, World!"), output_file_contents)
    }
}
