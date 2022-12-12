use crate::{
    deserialization_structures::SolutionFilterFile, error::SolutionError, structs::SolutionFilter,
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
        let pattern = path.to_str().ok_or_else(|| SolutionError::InvalidPath("".to_owned()))?;

        let glob = glob(pattern)?;

        let mut filters = vec![];

        for filter in glob.flatten() {
            if let Some(filter) = self.parse_slnf(&filter) {
                filters.push(filter);
            } else {
                println!("Could not parse solution filter {:?}", filter);
            }
        }

        Ok(filters)
    }

    fn parse_slnf(&self, filter_file: &Path) -> Option<SolutionFilter> {
        let name = filter_file.file_name()?.to_str()?.to_owned();
        let content = fs::read_to_string(filter_file).ok()?;

        let separator = format!("{}", std::path::MAIN_SEPARATOR);
        let content = content.replace("\\\\", &separator);

        if let Ok(solution_filter) = serde_json::from_str::<SolutionFilterFile>(&content) {
            let projects = solution_filter
                .solution
                .projects
                .iter()
                .filter_map(|d| PathBuf::from_str(d).ok())
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
