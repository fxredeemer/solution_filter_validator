use arguments::Arguments;
use clap::Parser;
use solution_filter_reader::SolutionFilterReader;
use solution_reader::SolutionReader;
use std::error::Error;

mod arguments;
mod deserialization_structures;
mod error;
mod solution_filter_reader;
mod solution_reader;
mod structs;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse();

    let solution_reader = SolutionReader::new(&arguments.sln_file);
    let solution_filter_reader = SolutionFilterReader::new(&arguments.base_path);

    let projects = solution_reader.get_projects()?;
    let filters = solution_filter_reader.get_solution_filters()?;

    println!("Solution Filters");

    for filter in filters {
        println!("{:?}", filter.path);
        for project in filter.projects {
            println!("{project:?}");
        }
    }

    println!();
    println!("Projects");

    for project in projects {
        println!("{}", project.name)
    }

    Ok(())
}
