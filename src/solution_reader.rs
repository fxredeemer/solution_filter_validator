use std::{error::Error, fs, path::{Path, PathBuf}};

use regex::Regex;

use crate::project::Project;

pub struct SolutionReader {
    path: PathBuf,
}

impl SolutionReader {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
        }
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, Box<dyn Error>> {
        let content = match fs::read_to_string(self.path.to_owned()) {
            Ok(content) => Ok(content),
            Err(_) => Err("Unable to read file contents".to_owned()),
        }?;

        let regex =
            Regex::new(r#"^Project\("\{(.+?)\}"\) = "(.+?)", "(.+?)", "\{(.+?)\}""#).unwrap();

        let mut projects = vec![];

        for line in content.lines() {
            if let Some(captures) = regex.captures(line) {
                let name = captures[2].to_owned();
                let path = captures[3].to_owned();

                projects.push(Project { name, path });
            }
        }

        Ok(projects)
    }
}
