use crate::{
    deserialization_structures::SolutionFilterFile, error::FileError, structs::SolutionFilter,
};
use glob::*;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
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
            .ok_or(FileError::InvalidPath("".to_owned()))?;

        let glob = glob(pattern)?;

        let mut filters = vec![];

        for entry in glob {
            if let Ok(filter) = entry {
                if let Some(filter) = self.parse_slnf(&filter) {
                    filters.push(filter);
                } else {
                    println!("Could not parse solution filter {:?}", filter);
                }
            }
        }

        Ok(filters)
    }

    fn parse_slnf(&self, filter_file: &Path) -> Option<SolutionFilter> {
        let name = filter_file.file_name()?.to_str()?.to_owned();
        let content = fs::read_to_string(filter_file.to_owned()).ok()?;

        if let Some(solution_filter) = serde_json::from_str::<SolutionFilterFile>(&content).ok() {
            let projects = solution_filter
                .solution
                .projects
                .iter()
                .filter_map(|d| PathBuf::from_str(&d).ok())
                .collect();

            return Some(SolutionFilter {
                name,
                path: filter_file.to_owned(),
                projects,
                solution_path: solution_filter.solution.path,
            });
        }

        None
    }
}
