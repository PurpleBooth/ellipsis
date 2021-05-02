use std::fs;
use std::os::unix::fs as unixfs;
use std::path::{Path, PathBuf};

use crate::domain;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Driver {}

impl Driver {
    pub fn new() -> Driver {
        Driver {}
    }
}

impl domain::Driver for Driver {
    fn copy(self, from: &Path, to: &Path) -> Result<Driver, domain::Error> {
        fs::copy(from, to)
            .map_err(|error| domain::Error::Copy(PathBuf::from(from), PathBuf::from(to), error))
            .map(|_| Driver::new())
    }

    fn link(self, from: &Path, to: &Path) -> Result<Driver, domain::Error> {
        unixfs::symlink(from, to)
            .map_err(|error| domain::Error::Link(PathBuf::from(from), PathBuf::from(to), error))
            .map(|_| Driver::new())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};

    use super::Driver as IoDriver;
    use crate::domain::Driver;

    #[test]
    fn copy_file() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        let mut file = File::create(&working_dir.join("in.txt")).unwrap();
        write!(file, "Hello, World!").unwrap();

        IoDriver::new()
            .copy(&working_dir.join("in.txt"), &working_dir.join("out.txt"))
            .unwrap();

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

        IoDriver::new()
            .link(&working_dir.join("in.txt"), &working_dir.join("out.txt"))
            .unwrap();

        let mut output_file_contents = String::new();
        File::open(working_dir.join("out.txt"))
            .unwrap()
            .read_to_string(&mut output_file_contents)
            .unwrap();

        assert_eq!(String::from("Hello, World!"), output_file_contents);
        assert_eq!(
            true,
            fs::symlink_metadata(working_dir.join("out.txt"))
                .unwrap()
                .file_type()
                .is_symlink()
        )
    }
}
