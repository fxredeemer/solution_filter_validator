use crate::error::SolutionError;
use glob::*;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub struct SolutionFiltersProvider {
    base_path: PathBuf,
}

impl SolutionFiltersProvider {
    pub fn new(base_path: &Path) -> Self {
        Self {
            base_path: base_path.to_owned(),
        }
    }

    pub fn get_solution_filter_files(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let path = Path::new(&self.base_path).join("**/*.slnf");
        let pattern = path
            .to_str()
            .ok_or_else(|| SolutionError::InvalidPath("".to_owned()))?;

        let glob = glob(pattern)?;

        return Ok(glob.flatten().collect());
    }
}
