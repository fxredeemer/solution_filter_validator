use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolutionError {
    #[error("The Path `{0}` is invalid")]
    InvalidPath(String),
    #[error("The Solution Filter `{0}` has wrongly referenced projects: \r\n{1}")]
    FaultyProjectReference(PathBuf, String),
    #[error("The Solution Filter `{0}` references a not existing solution: {1}")]
    InvalidSolutionReference(PathBuf, PathBuf),
}
