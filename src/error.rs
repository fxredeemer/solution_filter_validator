use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteFileError {
    #[error("The Path `{0}` is invalid")]
    InvalidPath(String),
}
