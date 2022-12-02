use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SolutionFilterFile{
    pub solution: SolutionFile,
}

#[derive(Deserialize, Debug)]
pub struct SolutionFile
{
    pub path: PathBuf,
    pub projects: Vec<String>
}
