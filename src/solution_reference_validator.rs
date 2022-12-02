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

        if expected_solution_path != solution.path {
            return Err(Box::new(SolutionError::InvalidSolutionReference(
                solution_filter.path.to_owned(),
                solution.path.to_owned(),
            )));
        }

        Ok(())
    }
}
