use crate::{deserialization_structures::SolutionFilterFile, structs::SolutionFilter};
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

pub struct SolutionFilterReader;

impl SolutionFilterReader {
    pub fn new() -> Self {
        Self
    }

    fn parse_slnf(&self, filter_file: &Path) -> Option<SolutionFilter> {
        let name = filter_file.file_name()?.to_str()?.to_owned();
        let content = fs::read_to_string(filter_file).ok()?;

        let regex = Regex::new(",\\s*\\]").unwrap();
        let content = regex.replace_all(&content, "]");

        return match serde_json::from_str::<SolutionFilterFile>(&content) {
            Ok(solution_filter) => {
                let projects = solution_filter
                    .solution
                    .projects
                    .iter()
                    .filter_map(|d| PathBuf::from_str(d).ok())
                    .collect();

                Some(SolutionFilter {
                    name,
                    path: filter_file.to_owned(),
                    projects,
                    solution_path: solution_filter.solution.path,
                })
            }
            Err(error) => {
                println!("error in parsing: {}", error);
                None
            }
        };
    }
}
