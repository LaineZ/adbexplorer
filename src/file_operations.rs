use std::{env, fs};

use anyhow::Result;

pub struct Local {
    working_directory: String,
}

pub trait FileOperations {
    fn get_files(&mut self) -> Result<Vec<String>>;
    fn change_directory_rel(&mut self, path: &str);
    fn level_up_files(&mut self) -> Result<Vec<String>>;
    fn is_directory(path: String) -> bool;
}

impl Local {
    pub fn new() -> Result<Local> {
        Ok(Local {
            working_directory: env::current_dir()?.to_string_lossy().into()
        })
    }
}

impl FileOperations for Local {
    fn get_files(&mut self) -> Result<Vec<String>> {
        let paths = fs::read_dir(".")?;
        let mut files = Vec::new();

        for path in paths {
            if let Some(filename) = path?.path().file_name() {
                files.push(filename.to_string_lossy().into());
            }
        }

        Ok(files)
    }

    // TODO: Make these functions work on Non-unix systems e.g Windows
    fn change_directory_rel(&mut self, path: &str) {
        self.working_directory = format!("{}{}", self.working_directory, path);
    }

    fn level_up_files(&mut self) -> Result<Vec<String>> {
        let mut splited_path = self.working_directory.split(" ").collect::<Vec<&str>>();
        splited_path.remove(splited_path.len() - 1);
        self.working_directory = splited_path.join("/");
        self.get_files()
    }

    // ESPECIALLY THIS
    fn is_directory(path: String) -> bool {
        path.ends_with("/")
    }
}
