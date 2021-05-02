use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

use crate::domain;
use crate::domain::DriverTypes;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigOuter {
    todo: Vec<ConfigOperation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CopyOptions {
    to: String,
    from: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ConfigOperation {
    #[serde(rename = "copy")]
    Copy { to: String, from: String },
    #[serde(rename = "link")]
    Link { to: String, from: String },
}

pub struct Config {
    pub driver: DriverTypes,
    pub operations: Vec<domain::Operation>,
}

impl TryFrom<&ArgMatches> for Config {
    type Error = Error;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let mut config_location = File::open(matches.value_of("config").unwrap())?;
        let mut config = String::new();
        config_location.read_to_string(&mut config)?;
        let home = matches.value_of("home").unwrap();
        let dry_run = matches.is_present("dry-run");

        let deserialized_point: ConfigOuter = serde_yaml::from_str(&config)?;
        let current_dir = env::current_dir()?;

        Ok(Config {
            driver: if dry_run {
                DriverTypes::Blackhole
            } else {
                DriverTypes::Io
            },
            operations: deserialized_point
                .todo
                .into_iter()
                .map(|operation| match operation {
                    ConfigOperation::Copy { to, from } => domain::Operation::Copy {
                        from: domain::OperationPath::new(&current_dir, Path::new(home), &from),
                        to: domain::OperationPath::new(&current_dir, Path::new(home), &to),
                    },
                    ConfigOperation::Link { to, from } => domain::Operation::Link {
                        from: domain::OperationPath::new(&current_dir, Path::new(home), &from),
                        to: domain::OperationPath::new(&current_dir, Path::new(home), &to),
                    },
                })
                .collect(),
        })
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("yaml parse error")]
    YamlParse(#[from] serde_yaml::Error),
    #[error("io error")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::env;
    use std::io::Write;

    use indoc::indoc;

    use crate::cli::app;
    use crate::domain;

    use super::Config;

    #[test]
    fn no_config_defined() {
        let args = app().get_matches_from(vec!["ellipsis"]);
        assert_eq!(Config::try_from(&args).is_err(), true)
    }

    #[test]
    fn copy_operation() {
        let home = tempfile::tempdir().unwrap();
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            indoc! {r#"
            ---
            todo:
            - copy:
                from: source.txt
                to: ~/destination.txt
        "#}
        )
        .unwrap();

        let args = app().get_matches_from(vec![
            "ellipsis",
            "--home",
            &home.path().display().to_string(),
            "--config",
            &tmpfile.path().display().to_string(),
        ]);
        assert_eq!(
            Config::try_from(&args).unwrap().operations,
            vec![domain::Operation::Copy {
                from: domain::OperationPath::new(
                    &env::current_dir().unwrap(),
                    &home.path().to_path_buf(),
                    "source.txt",
                ),
                to: domain::OperationPath::new(
                    &env::current_dir().unwrap(),
                    &home.path().to_path_buf(),
                    "~/destination.txt",
                ),
            }]
        )
    }

    #[test]
    fn link_operation() {
        let home = tempfile::tempdir().unwrap();
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            indoc! {r#"
            ---
            todo:
            - link:
                from: source.txt
                to: ~/destination.txt
        "#}
        )
        .unwrap();

        let args = app().get_matches_from(vec![
            "ellipsis",
            "--home",
            &home.path().display().to_string(),
            "--config",
            &tmpfile.path().display().to_string(),
        ]);
        assert_eq!(
            Config::try_from(&args).unwrap().operations,
            vec![domain::Operation::Link {
                from: domain::OperationPath::new(
                    &env::current_dir().unwrap(),
                    &home.path().to_path_buf(),
                    "source.txt",
                ),
                to: domain::OperationPath::new(
                    &env::current_dir().unwrap(),
                    &home.path().to_path_buf(),
                    "~/destination.txt",
                ),
            }]
        )
    }
}
