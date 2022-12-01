use std::{error::Error, iter::Filter};

use arguments::Arguments;
use clap::Parser;
use solution_filter_reader::SolutionFilterReader;
use solution_reader::SolutionReader;

mod arguments;
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

    for filter in filters {
        println!("{}", filter.name);
    }

    for project in projects {
        println!("{}", project.name)
    }

    Ok(())
}
