use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Copy {
        from: String,
        to: String,
        home: PathBuf,
        working_dir: PathBuf,
    },
}
