use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;

use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

use crate::domain;
use std::env;

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
}

pub(crate) struct Config {
    pub(crate) operations: Vec<domain::Operation>,
}

impl TryFrom<&ArgMatches> for Config {
    type Error = Error;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let mut config_location = File::open(matches.value_of("config").unwrap())?;
        let mut config = String::new();
        config_location.read_to_string(&mut config)?;
        let home = matches.value_of("home").unwrap();

        let deserialized_point: ConfigOuter = serde_yaml::from_str(&config)?;
        let current_dir = env::current_dir()?;

        Ok(Config {
            operations: deserialized_point
                .todo
                .into_iter()
                .map(|operation| match operation {
                    ConfigOperation::Copy { to, from } => domain::Operation::Copy {
                        from,
                        to,
                        home: home.into(),
                        working_dir: current_dir.clone(),
                    },
                })
                .collect(),
        })
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    YamlParse(#[from] serde_yaml::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::io::Write;

    use indoc::indoc;

    use crate::cli::app;
    use crate::domain;

    use super::Config;
    use std::env;

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
                from: "source.txt".into(),
                to: "~/destination.txt".into(),
                home: home.into_path(),
                working_dir: env::current_dir().unwrap()
            }]
        )
    }
}
