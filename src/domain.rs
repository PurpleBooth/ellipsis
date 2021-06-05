use core::result::Result;
use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error as ThisError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Copy {
        from: OperationPath,
        to: OperationPath,
    },
    Link {
        from: OperationPath,
        to: OperationPath,
        overwrite: bool,
    },
    Exec {
        working_dir: PathBuf,
        command: String,
        args: Vec<String>,
    },
}

impl Operation {
    pub(crate) fn new_copy(home: &str, current_dir: &Path, to: &str, from: &str) -> Operation {
        Operation::Copy {
            from: OperationPath::new(current_dir, Path::new(home), from),
            to: OperationPath::new(current_dir, Path::new(home), to),
        }
    }

    pub(crate) fn new_link(
        home: &str,
        current_dir: &Path,
        to: &str,
        from: &str,
        overwrite: bool,
    ) -> Operation {
        Operation::Link {
            from: OperationPath::new(current_dir, Path::new(home), from),
            to: OperationPath::new(current_dir, Path::new(home), to),
            overwrite,
        }
    }

    pub(crate) fn new_exec(current_dir: &Path, command: String, args: Vec<String>) -> Operation {
        Operation::Exec {
            working_dir: current_dir.to_path_buf(),
            command,
            args,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationPath {
    pub location: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationCommand {
    pub run: String,
    pub shell: String,
}

impl OperationPath {
    pub(crate) fn new(working_dir: &Path, home: &Path, location: &str) -> OperationPath {
        OperationPath {
            location: match location.strip_prefix("~/") {
                Some(to) => OperationPath::canonical_path(home, Path::new(to)),
                None => OperationPath::canonical_path(working_dir, Path::new(&location)),
            },
        }
    }

    fn canonical_path(working_dir: &Path, from: &Path) -> PathBuf {
        if from.is_relative() {
            working_dir.join(&from)
        } else {
            from.into()
        }
    }
}

pub enum DriverTypes {
    Blackhole,
    Io,
}

pub trait Driver<NewSelf = Self> {
    fn copy(self, from: &Path, to: &Path) -> Result<NewSelf, Error>;
    fn link(self, from: &Path, to: &Path, overwrite: bool) -> Result<NewSelf, Error>;
    fn exec(self, working_dir: &Path, command: &str, args: &[String]) -> Result<NewSelf, Error>;
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("copy from `{0}` to `{1}` failed")]
    Copy(PathBuf, PathBuf, #[source] io::Error),
    #[error("link from `{0}` to `{1}` failed")]
    Link(PathBuf, PathBuf, #[source] io::Error),
    #[error("exec `{0} {1}` in {2} failed")]
    Exec(String, String, PathBuf, #[source] io::Error),
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::domain::OperationPath;

    #[test]
    fn relative_canonical_to() {
        assert_eq!(
            Path::new("/tmp/example.txt"),
            OperationPath::new(
                Path::new("/tmp"),
                Path::new("/something/else"),
                "example.txt",
            )
            .location
        );
    }

    #[test]
    fn absolute_canonical_to() {
        assert_eq!(
            Path::new("/tmp/example.txt"),
            OperationPath::new(
                Path::new("/something/else"),
                Path::new("/something/else"),
                "/tmp/example.txt",
            )
            .location
        );
    }

    #[test]
    fn home_magic_canonical_to() {
        assert_eq!(
            Path::new("/tmp/example.txt"),
            OperationPath::new(
                Path::new("/tmp"),
                Path::new("/something/else"),
                "/tmp/example.txt",
            )
            .location
        );
    }
}
