use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;

use crate::structs::{Project, Solution};

pub struct SolutionReader {
    path: PathBuf,
}

impl SolutionReader {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
        }
    }

    pub fn read_solution(&self) -> Result<Solution, Box<dyn Error>> {
        let content = match fs::read_to_string(&self.path) {
            Ok(content) => Ok(content),
            Err(_) => Err("Unable to read file contents".to_owned()),
        }?;

        let regex =
            Regex::new(r#"^Project\("\{(.+?)\}"\) = "(.+?)", "(.+?)", "\{(.+?)\}""#).unwrap();

        let mut projects = vec![];

        for line in content.lines() {
            if let Some(captures) = regex.captures(line) {
                let name = captures[2].to_owned();

                let mut path = PathBuf::new();
                path.push(&captures[3]);

                projects.push(Project { name, path });
            }
        }

        Ok(Solution {
            path: self.path.to_owned(),
            projects,
        })
    }
}
