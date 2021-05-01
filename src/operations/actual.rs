use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error as ThisError;

use crate::config::Config;
use crate::domain::Operation;

pub(crate) fn run(input: Config) -> Result<(), Error> {
    for operation in input.operations {
        match operation {
            Operation::Copy {
                from,
                to,
                home,
                working_dir,
            } => {
                let canonical_from = canonical_from(&working_dir, &from);
                let canonical_to = canonical_to(&home, &working_dir, &to);
                fs::copy(canonical_from.clone(), canonical_to.clone())
                    .map_err(|error| Error::Copy(from, to, working_dir, home, error.to_string()))
                    .map(|_| ())?
            }
        }
    }

    Ok(())
}

fn canonical_from(working_dir: &Path, from: &str) -> PathBuf {
    if Path::new(&from).is_relative() {
        working_dir.join(&from)
    } else {
        Path::new(&from).into()
    }
}

fn canonical_path(working_dir: &Path, from: &Path) -> PathBuf {
    if from.is_relative() {
        working_dir.join(&from)
    } else {
        from.into()
    }
}

fn canonical_to(home_dir: &Path, working_dir: &Path, to: &str) -> PathBuf {
    match to.strip_prefix("~/") {
        Some(to) => canonical_path(working_dir, &home_dir.join(&to)),
        None => canonical_path(working_dir, &Path::new(to)),
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(
        "copy from `{0}` to `{1}` with working directory `{2}` and home `{3}` failed with `{4}`"
    )]
    Copy(String, String, PathBuf, PathBuf, String),
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;

    use crate::config::Config;
    use crate::domain;
    use crate::operations::actual::{canonical_from, canonical_to, run};

    #[test]
    fn relative_canonical_from() {
        assert_eq!(
            Path::new("/tmp/example.txt"),
            canonical_from(&Path::new("/tmp"), "example.txt")
        )
    }

    #[test]
    fn absolute_canonical_from() {
        assert_eq!(
            Path::new("/tmp/example.txt"),
            canonical_from(&Path::new("/something/else"), "/tmp/example.txt")
        )
    }

    #[test]
    fn relative_canonical_to() {
        assert_eq!(
            Path::new("/tmp/example.txt"),
            canonical_to(
                &Path::new("/something/else"),
                &Path::new("/tmp"),
                "example.txt"
            )
        )
    }

    #[test]
    fn absolute_canonical_to() {
        assert_eq!(
            Path::new("/tmp/example.txt"),
            canonical_to(
                &Path::new("/something/else"),
                &Path::new("/something/else"),
                "/tmp/example.txt"
            )
        )
    }

    #[test]
    fn home_magic_canonical_to() {
        assert_eq!(
            Path::new("/tmp/example.txt"),
            canonical_to(
                &Path::new("/tmp"),
                &Path::new("/something/else"),
                "/tmp/example.txt"
            )
        )
    }

    #[test]
    fn copy_to_working_dir_no_magic() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        let mut file = File::create(&working_dir.join("in.txt")).unwrap();
        write!(file, "Hello, World!").unwrap();

        let input = Config {
            operations: vec![domain::Operation::Copy {
                home: tempfile::tempdir().unwrap().into_path(),
                to: "out.txt".into(),
                from: "in.txt".into(),
                working_dir: working_dir.clone(),
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
                home: home.clone(),
                to: "~/out.txt".into(),
                from: "in.txt".into(),
                working_dir,
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
