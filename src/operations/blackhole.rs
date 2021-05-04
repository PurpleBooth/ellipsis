use std::path::Path;

use crate::domain;
use crate::domain::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Driver {
    pub log: Vec<(String, String)>,
}

impl Driver {
    #[allow(dead_code)]
    pub fn new() -> Driver {
        Driver { log: Vec::new() }
    }

    pub fn log(&mut self, kind: String, message: String) {
        self.log.push((kind, message))
    }
}

impl domain::Driver for Driver {
    fn copy(mut self, from: &Path, to: &Path) -> Result<Driver, domain::Error> {
        self.log("copy".into(), format!("{:?} -> {:?}", from, to));
        Ok(self)
    }

    fn link(mut self, from: &Path, to: &Path, overwrite: bool) -> Result<Driver, Error> {
        self.log(
            "link".into(),
            format!("{:?} -> (overwriting: {}) {:?}", from, overwrite, to),
        );
        Ok(self)
    }

    fn exec(
        mut self,
        working_dir: &Path,
        command: &str,
        args: &[std::string::String],
    ) -> Result<Self, Error> {
        self.log(
            "exec".into(),
            format!("in {:?} {:?} {:?}", working_dir, command, args),
        );
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Driver as BlackholeDriver;
    use crate::domain::Driver;

    #[test]
    fn exec_file() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        let driver = BlackholeDriver::new()
            .exec(
                &working_dir,
                "bash",
                &["-c".into(), "echo hello > out.txt".into()],
            )
            .unwrap();

        assert_eq!(
            vec![(
                String::from("exec"),
                format!(
                    "in {:?} \"bash\" [\"-c\", \"echo hello > out.txt\"]",
                    working_dir.display()
                )
            )],
            driver.log
        )
    }

    #[test]
    fn copy_file() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        let driver = BlackholeDriver::new()
            .copy(&working_dir.join("in.txt"), &working_dir.join("out.txt"))
            .unwrap();

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
        )
    }

    #[test]
    fn link_file() {
        let working_dir = tempfile::tempdir().unwrap().into_path();
        let driver = BlackholeDriver::new()
            .link(
                &working_dir.join("in.txt"),
                &working_dir.join("out.txt"),
                false,
            )
            .unwrap();

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
        )
    }
}
