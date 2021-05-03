use std::fs;
use std::os::unix::fs as unixfs;
use std::path::{Path, PathBuf};

use crate::domain;
use crate::domain::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Driver {}

impl Driver {
    pub fn new() -> Driver {
        Driver {}
    }
}

impl domain::Driver for Driver {
    fn copy(self, from: &Path, to: &Path) -> Result<Driver, domain::Error> {
        Driver::make_containing_directory(to).map_err(Driver::map_to_domain_error(
            from.to_path_buf(),
            to.to_path_buf(),
        ))?;

        fs::copy(from, to)
            .map_err(|error| domain::Error::Copy(PathBuf::from(from), PathBuf::from(to), error))
            .map(|_| Driver::new())
    }

    fn link(self, from: &Path, to: &Path, overwrite: bool) -> Result<Driver, domain::Error> {
        if overwrite {
            Driver::delete_real_file_if_exists(to).map_err(Driver::map_to_domain_error(
                from.to_path_buf(),
                to.to_path_buf(),
            ))?;
        }

        Driver::make_containing_directory(to).map_err(Driver::map_to_domain_error(
            from.to_path_buf(),
            to.to_path_buf(),
        ))?;

        unixfs::symlink(from, to)
            .map_err(Driver::map_to_domain_error(
                from.to_path_buf(),
                to.to_path_buf(),
            ))
            .map(|_| Driver::new())
    }
}

impl Driver {
    fn map_to_domain_error(from: PathBuf, to: PathBuf) -> impl FnOnce(std::io::Error) -> Error {
        |error| domain::Error::Link(from, to, error)
    }
}

impl Driver {
    fn delete_real_file_if_exists(path: &Path) -> Result<(), std::io::Error> {
        if !path.exists() {
            return Ok(());
        }

        if fs::metadata(path)?.file_type().is_file() {
            fs::remove_file(path).map(|_| ())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;

    use super::Driver as IoDriver;
    use crate::domain::Driver;

    #[test]
    fn copy_file() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        write_file(&working_dir.join("in.txt"), "Hello, World!");

        IoDriver::new()
            .copy(&working_dir.join("in.txt"), &working_dir.join("out.txt"))
            .unwrap();
        let output_file_contents = read_file(&working_dir.join("out.txt"));
        assert_eq!(String::from("Hello, World!"), output_file_contents)
    }

    #[test]
    fn copy_file_into_deep_dir() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        write_file(&working_dir.join("in.txt"), "Hello, World!");

        IoDriver::new()
            .copy(
                &working_dir.join("in.txt"),
                &working_dir
                    .join("a")
                    .join("deep")
                    .join("dir")
                    .join("out.txt"),
            )
            .unwrap();
        let output_file_contents = read_file(
            &working_dir
                .join("a")
                .join("deep")
                .join("dir")
                .join("out.txt"),
        );
        assert_eq!(String::from("Hello, World!"), output_file_contents)
    }

    #[test]
    fn link_file() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        write_file(&working_dir.join("in.txt"), "Hello, World!");

        IoDriver::new()
            .link(
                &working_dir.join("in.txt"),
                &working_dir.join("out.txt"),
                false,
            )
            .unwrap();

        let output_file_contents = read_file(&working_dir.join("out.txt"));
        assert_eq!(String::from("Hello, World!"), output_file_contents);
        assert!(fs::symlink_metadata(working_dir.join("out.txt"))
            .unwrap()
            .file_type()
            .is_symlink())
    }

    #[test]
    fn link_file_into_deep_dir() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        write_file(&working_dir.join("in.txt"), "Hello, World!");

        IoDriver::new()
            .link(
                &working_dir.join("in.txt"),
                &working_dir
                    .join("a")
                    .join("deep")
                    .join("dir")
                    .join("out.txt"),
                false,
            )
            .unwrap();

        let output_file_contents = read_file(
            &working_dir
                .join("a")
                .join("deep")
                .join("dir")
                .join("out.txt"),
        );
        assert_eq!(String::from("Hello, World!"), output_file_contents);
        assert!(fs::symlink_metadata(
            working_dir
                .join("a")
                .join("deep")
                .join("dir")
                .join("out.txt")
        )
        .unwrap()
        .file_type()
        .is_symlink())
    }

    #[test]
    fn link_file_and_overwrite() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        write_file(&working_dir.join("in.txt"), "Hello, World!");
        write_file(&working_dir.join("out.txt"), "I exist");

        IoDriver::new()
            .link(
                &working_dir.join("in.txt"),
                &working_dir.join("out.txt"),
                true,
            )
            .unwrap();

        let output_file_contents = read_file(&working_dir.join("out.txt"));

        assert_eq!(String::from("Hello, World!"), output_file_contents);
        assert!(fs::symlink_metadata(working_dir.join("out.txt"))
            .unwrap()
            .file_type()
            .is_symlink())
    }

    #[test]
    fn link_file_and_do_not_overwrite() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        write_file(&working_dir.join("in.txt"), "Hello, World!");
        write_file(&working_dir.join("out.txt"), "I exist");

        assert!(IoDriver::new()
            .link(
                &working_dir.join("in.txt"),
                &working_dir.join("out.txt"),
                false,
            )
            .is_err());
    }

    fn read_file(working_dir: &Path) -> String {
        let mut output_file_contents = String::new();
        File::open(working_dir)
            .unwrap()
            .read_to_string(&mut output_file_contents)
            .unwrap();
        output_file_contents
    }

    fn write_file(working_dir: &Path, text: &str) {
        let mut file = File::create(&working_dir).unwrap();
        write!(file, "{}", text).unwrap();
    }
}

impl Driver {
    fn make_containing_directory(to: &Path) -> Result<(), std::io::Error> {
        if let Some(path) = to.parent().filter(|x| !x.exists()) {
            fs::create_dir_all(path)?
        };

        Ok(())
    }
}
