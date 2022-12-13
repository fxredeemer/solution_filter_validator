use crate::{error::SolutionError, helpers::try_get_containing_folder, structs::SolutionFilter};
use std::{error::Error, fs::canonicalize, path::PathBuf};

pub struct SolutionReferenceValidator;

impl SolutionReferenceValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_solution_reference(
        &self,
        solution_filter: &SolutionFilter,
    ) -> Result<(), Box<dyn Error>> {
        let folder = try_get_containing_folder(&solution_filter.path)?;

        let expected_solution_path = PathBuf::new()
            .join(folder)
            .join(&solution_filter.solution_path);

        match canonicalize(&expected_solution_path) {
            Ok(_) => Ok(()),
            Err(_) => Err(Box::new(SolutionError::InvalidSolutionReference(
                solution_filter.path.to_owned(),
                solution_filter.solution_path.to_owned(),
            ))),
        }
    }
}
