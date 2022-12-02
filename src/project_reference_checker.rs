use std::{error::Error, fs::canonicalize, path::{PathBuf, Path}};

use crate::{
    error::FileError,
    structs::{Solution, SolutionFilter},
};

pub struct ProjectReferenceChecker;

impl ProjectReferenceChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_references(
        &self,
        _solution: &Solution,
        solution_filter: SolutionFilter,
    ) -> Result<(), Box<dyn Error>> {
        println!("Path: {:?}", solution_filter.path);

        let solution_filter_containing_folder = try_get_containing_folder(&solution_filter.path)?;

        let solution_path = solution_filter_containing_folder.join(solution_filter.solution_path);

        println!("Solutionpath: {:?}", solution_path);

        for project in solution_filter.projects {
            let mut project_path = PathBuf::new();

            
            let solution_filter_containing_folder = try_get_containing_folder(&solution_filter.path)?;
            //project_path = project_path.join(path.to_owned());
            project_path = project_path.join(project);

            println!("Relative Project Path: {:?}", project_path);

            let absolute_project_path = canonicalize(project_path)?;

            println!("Absolute Project Path: {:?}", absolute_project_path);
        }

        Ok(())
    }
}


fn try_get_containing_folder(path : &Path) -> Result<PathBuf, Box<dyn Error>>
{
    path
        .parent()
        .map(|d| d.to_owned())
        .ok_or(Box::new(FileError::InvalidPath("".to_owned())))
}