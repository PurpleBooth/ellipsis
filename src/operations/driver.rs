use core::result::Result;
use std::path::{Path, PathBuf};
use thiserror::Error as ThisError;

pub trait Driver<NewSelf = Self> {
    fn copy(self, from: &Path, to: &Path) -> Result<NewSelf, Error>;
    fn link(self, from: &Path, to: &Path) -> Result<NewSelf, Error>;
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("copy from `{0}` to `{1}` failed with `{2}`")]
    Copy(PathBuf, PathBuf, String),
    #[error("link from `{0}` to `{1}` failed with `{2}`")]
    Link(PathBuf, PathBuf, String),
}

pub enum Types {
    Blackhole,
    Io,
}
