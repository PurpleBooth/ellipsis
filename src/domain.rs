use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Copy {
        from: OperationPath,
        to: OperationPath,
    },
    Link {
        from: OperationPath,
        to: OperationPath,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationPath {
    pub location: PathBuf,
}

impl OperationPath {
    pub(crate) fn new(working_dir: &Path, home: &Path, location: &str) -> OperationPath {
        OperationPath {
            location: match location.strip_prefix("~/") {
                Some(to) => canonical_path(&home, Path::new(to)),
                None => canonical_path(&working_dir, &Path::new(&location)),
            },
        }
    }
}

fn canonical_path(working_dir: &Path, from: &Path) -> PathBuf {
    if from.is_relative() {
        working_dir.join(&from)
    } else {
        from.into()
    }
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
        )
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
        )
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
        )
    }
}
