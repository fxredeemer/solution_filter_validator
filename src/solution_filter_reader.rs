use glob::*;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

use crate::{
    error::WriteFileError,
    structs::{Project, SolutionFilter},
};

pub struct SolutionFilterReader {
    base_path: PathBuf,
}

impl SolutionFilterReader {
    pub fn new(base_path: &Path) -> Self {
        Self {
            base_path: base_path.to_owned(),
        }
    }

    pub fn get_solution_filters(&self) -> Result<Vec<SolutionFilter>, Box<dyn Error>> {
        let path = Path::new(&self.base_path).join("**/*.slnf");
        let pattern = path
            .to_str()
            .ok_or(WriteFileError::InvalidPath("".to_owned()))?;

        let glob = glob(pattern)?;

        let mut filters = vec![];

        for entry in glob {
            if let Ok(filter) = entry {
                if let Some(filter) = self.parse_slnf(&filter) {
                    filters.push(filter);
                }
            }
        }

        Ok(filters)
    }

    fn parse_slnf(&self, filter_file: &Path) -> Option<SolutionFilter> {
        None
    }
}
