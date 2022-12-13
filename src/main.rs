use arguments::Arguments;
use clap::Parser;
use project_reference_checker::ProjectReferenceChecker;
use solution_filter_reader::SolutionFilterReader;
use solution_reference_validator::SolutionReferenceValidator;

mod arguments;
mod deserialization_structures;
mod error;
mod helpers;
mod project_reference_checker;
mod solution_filter_reader;
mod solution_reference_validator;
mod structs;

fn main() -> Result<(), ()> {
    match execute() {
        Ok(_) => Ok(()),
        Err(error) => {
            println!("{error}");
            Err(())
        }
    }
}

fn execute() -> Result<(), String> {
    let arguments = Arguments::parse();

    let solution_filter_reader = SolutionFilterReader::new(&arguments.base_path);
    let reference_checker = ProjectReferenceChecker::new();
    let reference_validator = SolutionReferenceValidator::new();

    let filters = solution_filter_reader
        .get_solution_filters()
        .map_err(|err| format!("{err}"))?;

    let mut errors = vec![];

    for filter in filters {
        if let Err(error) = reference_checker.validate_references(&filter) {
            errors.push(error);
        }

        if let Err(error) = reference_validator.validate_solution_reference(&filter) {
            errors.push(error)
        }
    }

    if !errors.is_empty() {
        let messages: Vec<String> = errors.iter().map(|d| format!("{}", d)).collect();
        let message = messages.join("\r\n");

        return Err(message);
    }

    Ok(())
}
