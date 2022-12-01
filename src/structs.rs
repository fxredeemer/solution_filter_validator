use std::path::PathBuf;

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub struct SolutionFilter {
    pub name: String,
    pub path: String,
    pub projects: Vec<PathBuf>
}
