use arguments::Arguments;
use clap::Parser;
use project_reference_checker::ProjectReferenceChecker;
use solution_filter_reader::SolutionFilterReader;
use solution_reader::SolutionReader;
use std::error::Error;

mod arguments;
mod deserialization_structures;
mod error;
mod solution_filter_reader;
mod solution_reader;
mod structs;
mod project_reference_checker;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse();

    let solution_reader = SolutionReader::new(&arguments.sln_file);
    let solution_filter_reader = SolutionFilterReader::new(&arguments.base_path);
    let reference_checker = ProjectReferenceChecker::new();

    let solution = solution_reader.read_solution()?;
    let filters = solution_filter_reader.get_solution_filters()?;

    for filter in filters {
        reference_checker.validate_references(&solution, filter)?;
    }

    Ok(())
}
