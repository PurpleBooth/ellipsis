use std::fs;
use std::os::unix::fs as unixfs;
use std::path::PathBuf;

use thiserror::Error as ThisError;

use crate::config::Config;
use crate::domain::Operation;

pub(crate) fn run(input: Config) -> Result<(), Error> {
    for operation in input.operations {
        match operation {
            Operation::Copy { from, to } => fs::copy(from.location.clone(), to.location.clone())
                .map_err(|error| {
                    Error::Copy(
                        from.location.clone(),
                        to.location.clone(),
                        error.to_string(),
                    )
                })
                .map(|_| ())?,
            Operation::Link { from, to } => {
                unixfs::symlink(from.location.clone(), to.location.clone())
                    .map_err(|error| {
                        Error::Link(
                            from.location.clone(),
                            to.location.clone(),
                            error.to_string(),
                        )
                    })
                    .map(|_| ())?
            }
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
    #[error("link from `{0}` to `{1}` failed with `{2}`")]
    Link(PathBuf, PathBuf, String),
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};

    use crate::config::Config;
    use crate::domain;
    use crate::domain::OperationPath;
    use crate::operations::actual::run;
    use std::fs;

    #[test]
    fn copy_file() {
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
    fn link_file() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        let mut file = File::create(&working_dir.join("in.txt")).unwrap();
        write!(file, "Hello, World!").unwrap();

        let input = Config {
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

        run(input).unwrap();

        let mut output_file_contents = String::new();
        let out_file_path = working_dir.join("out.txt");
        File::open(out_file_path.clone())
            .unwrap()
            .read_to_string(&mut output_file_contents)
            .unwrap();

        assert_eq!(String::from("Hello, World!"), output_file_contents);
        assert_eq!(
            true,
            fs::symlink_metadata(out_file_path)
                .unwrap()
                .file_type()
                .is_symlink()
        )
    }
}
