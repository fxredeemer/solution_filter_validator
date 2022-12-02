use crate::{error::SolutionError, helpers::try_get_containing_folder, structs::SolutionFilter};
use std::{error::Error, path::PathBuf};

pub struct ProjectReferenceChecker;

impl ProjectReferenceChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_references(
        &self,
        solution_filter: &SolutionFilter,
    ) -> Result<(), Box<dyn Error>> {
        let solution_filter_containing_folder = try_get_containing_folder(&solution_filter.path)?;

        let mut not_existing = vec![];

        for project in &solution_filter.projects {
            let project_path = PathBuf::new()
                .join(solution_filter_containing_folder.clone())
                .join(project);

            let project_exists = project_path.exists();

            if !project_exists {
                not_existing.push(project_path);
            }
        }

        if !not_existing.is_empty() {
            let inexisting_projects = not_existing
                .iter()
                .filter_map(|d| d.to_str())
                .collect::<Vec<&str>>()
                .join("\r\n");

            return Err(Box::new(SolutionError::FaultyProjectReference(
                solution_filter.path.to_owned(),
                inexisting_projects,
            )));
        }

        Ok(())
    }
}
