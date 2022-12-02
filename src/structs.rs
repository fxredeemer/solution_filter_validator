use std::path::PathBuf;

#[derive(Debug)]
pub struct Solution {
    pub path: PathBuf,
    pub projects: Vec<Project>
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct SolutionFilter {
    pub name: String,
    pub path: PathBuf,
    pub solution_path: PathBuf,
    pub projects: Vec<PathBuf>
}