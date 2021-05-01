use core::result::Result;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error as ThisError;

pub trait Driver<NewSelf = Self> {
    fn copy(self, from: &Path, to: &Path) -> Result<NewSelf, Error>;
    fn link(self, from: &Path, to: &Path) -> Result<NewSelf, Error>;
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("copy from `{0}` to `{1}` failed")]
    Copy(PathBuf, PathBuf, #[source] io::Error),
    #[error("link from `{0}` to `{1}` failed")]
    Link(PathBuf, PathBuf, #[source] io::Error),
}

pub enum Types {
    Blackhole,
    Io,
}
