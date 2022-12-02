use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("The Path `{0}` is invalid")]
    InvalidPath(String),
    #[error("The Solution Filter `{0}` has multiple wrongly referenced projects: {1}")]
    FaultySolutionFilter(PathBuf, String),
}
