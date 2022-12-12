use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    #[clap(short = 's', long = "sln")]
    pub sln_file: PathBuf,
    #[clap(short = 'p', long = "path")]
    pub base_path: PathBuf,
}
