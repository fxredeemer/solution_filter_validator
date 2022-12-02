use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("The Path `{0}` is invalid")]
    InvalidPath(String),
}
