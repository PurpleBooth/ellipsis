use std::path::Path;

use crate::operations::driver;

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

impl driver::Driver for Driver {
    fn copy(mut self, from: &Path, to: &Path) -> Result<Driver, driver::Error> {
        self.log("copy".into(), format!("{:?} -> {:?}", from, to));
        Ok(self)
    }

    fn link(mut self, from: &Path, to: &Path) -> Result<Driver, driver::Error> {
        self.log("link".into(), format!("{:?} -> {:?}", from, to));
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::operations::driver::Driver;

    use super::Driver as BlackholeDriver;

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
            .link(&working_dir.join("in.txt"), &working_dir.join("out.txt"))
            .unwrap();

        assert_eq!(
            vec![(
                String::from("link"),
                format!(
                    "{:?} -> {:?}",
                    working_dir.join("in.txt"),
                    working_dir.join("out.txt")
                )
            )],
            driver.log
        )
    }
}
