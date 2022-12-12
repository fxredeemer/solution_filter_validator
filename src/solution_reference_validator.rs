use std::{error::Error, path::PathBuf};

use crate::{
    error::SolutionError,
    helpers::try_get_containing_folder,
    structs::{Solution, SolutionFilter},
};

pub struct SolutionReferenceValidator;

impl SolutionReferenceValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_solution_reference(
        &self,
        solution: &Solution,
        solution_filter: &SolutionFilter,
    ) -> Result<(), Box<dyn Error>> {
        let folder = try_get_containing_folder(&solution_filter.path)?;

        let expected_solution_path = PathBuf::new()
            .join(folder)
            .join(&solution_filter.solution_path);

        let expected_solution_path = std::fs::canonicalize(expected_solution_path)?;
        match std::fs::canonicalize(&solution.path) {
            Ok(actual_solution_path) => {
                if expected_solution_path != actual_solution_path {
                    return Err(Box::new(SolutionError::InvalidSolutionReference(
                        solution_filter.path.to_owned(),
                        solution.path.to_owned(),
                    )));
                }
                Ok(())
            }
            Err(_) => Err(Box::new(SolutionError::InvalidSolutionReference(
                solution_filter.path.to_owned(),
                solution.path.to_owned(),
            ))),
        }
    }
}
