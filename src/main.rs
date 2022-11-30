use std::error::Error;

use arguments::Arguments;
use clap::Parser;
use solution_reader::SolutionReader;

mod arguments;
mod project;
mod solution_reader;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse();

    let solution_reader = SolutionReader::new(&arguments.sln_file);

    let projects = solution_reader.get_projects()?;

    for project in projects {
        println!("{}", project.name)
    }

    Ok(())
}
