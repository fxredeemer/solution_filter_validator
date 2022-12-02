use std::{
    error::Error,
    path::{Path, PathBuf},
};

use crate::error::SolutionError;

pub fn try_get_containing_folder(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let path = path
        .parent()
        .map(|d| d.to_owned())
        .ok_or_else(|| Box::new(SolutionError::InvalidPath("".to_owned())))?;

    Ok(path)
}
