use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SolutionFilterFile{
    pub solution: Solution,
}

#[derive(Deserialize, Debug)]
pub struct Solution
{
    pub path: PathBuf,
    pub projects: Vec<String>
}
