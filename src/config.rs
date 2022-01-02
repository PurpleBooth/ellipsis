use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::Read;

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
enum ConfigOperation {
    #[serde(rename = "copy")]
    Copy { to: String, from: String },
    #[serde(rename = "exec")]
    Exec { command: String, args: Vec<String> },
    #[serde(rename = "link")]
    Link {
        to: String,
        from: String,
        #[serde(default)]
        overwrite: bool,
    },
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

        Ok(Self {
            driver: if dry_run {
                DriverTypes::Blackhole
            } else {
                DriverTypes::Io
            },
            operations: deserialized_point
                .todo
                .into_iter()
                .map(|operation| match operation {
                    ConfigOperation::Copy { to, from } => {
                        domain::Operation::new_copy(home, &current_dir, &to, &from)
                    }
                    ConfigOperation::Link {
                        to,
                        from,
                        overwrite,
                    } => domain::Operation::new_link(home, &current_dir, &to, &from, overwrite),
                    ConfigOperation::Exec { command, args } => {
                        domain::Operation::new_exec(&current_dir, command, args)
                    }
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
    use tempfile::TempDir;

    use super::Config;
    use crate::cli::app;
    use crate::domain;
    use crate::domain::Operation;

    #[test]
    fn no_config_defined() {
        let args = app().get_matches_from(vec!["ellipsis"]);
        assert!(Config::try_from(&args).is_err());
    }

    #[test]
    fn exec_operation() {
        let home = tempfile::tempdir().unwrap();
        assert_yaml_parsing(
            indoc! {r#"
            ---
            todo:
            - exec:
                command: echo
                args: [hello]
        "#},
            &[domain::Operation::Exec {
                working_dir: env::current_dir().unwrap(),
                command: "echo".into(),
                args: vec!["hello".into()],
            }],
            &home,
        );
    }

    #[test]
    fn copy_operation() {
        let home = tempfile::tempdir().unwrap();
        assert_yaml_parsing(
            indoc! {r#"
            ---
            todo:
            - copy:
                from: source.txt
                to: ~/destination.txt
        "#},
            &[domain::Operation::Copy {
                from: domain::OperationPath::new(
                    &env::current_dir().unwrap(),
                    home.path(),
                    "source.txt",
                ),
                to: domain::OperationPath::new(
                    &env::current_dir().unwrap(),
                    home.path(),
                    "~/destination.txt",
                ),
            }],
            &home,
        );
    }

    #[test]
    fn link_operation() {
        let home = tempfile::tempdir().unwrap();

        assert_yaml_parsing(
            indoc! {r#"
            ---
            todo:
            - link:
                from: source.txt
                to: ~/destination.txt
        "#},
            &[domain::Operation::Link {
                from: domain::OperationPath::new(
                    &env::current_dir().unwrap(),
                    home.path(),
                    "source.txt",
                ),
                to: domain::OperationPath::new(
                    &env::current_dir().unwrap(),
                    home.path(),
                    "~/destination.txt",
                ),
                overwrite: false,
            }],
            &home,
        );
    }

    #[test]
    fn link_and_overwrite() {
        let home = tempfile::tempdir().unwrap();

        assert_yaml_parsing(
            indoc! {r#"
            ---
            todo:
            - link:
                from: source.txt
                to: ~/destination.txt
                overwrite: true
        "#},
            &[domain::Operation::Link {
                from: domain::OperationPath::new(
                    &env::current_dir().unwrap(),
                    home.path(),
                    "source.txt",
                ),
                to: domain::OperationPath::new(
                    &env::current_dir().unwrap(),
                    home.path(),
                    "~/destination.txt",
                ),
                overwrite: true,
            }],
            &home,
        );
    }

    fn assert_yaml_parsing(yaml: &str, expected: &[Operation], home: &TempDir) {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "{}", yaml).unwrap();

        let args = app().get_matches_from(vec![
            "ellipsis",
            "--home",
            &home.path().display().to_string(),
            "--config",
            &tmpfile.path().display().to_string(),
        ]);
        assert_eq!(Config::try_from(&args).unwrap().operations, expected);
    }
}
