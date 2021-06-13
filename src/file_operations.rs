use std::{env, fs};

use anyhow::Result;

pub struct Local {
    working_directory: String,
}

pub trait FileOperations {
    fn get_files(&mut self) -> Result<Vec<String>>;
    fn change_directory_rel(&mut self, path: &str);
    fn level_up_files(&mut self) -> Result<Vec<String>>;
    fn is_directory(&self, path: String) -> bool;
    fn get_working_directory(&self) -> &str;
}

impl Local {
    pub fn new() -> Result<Local> {
        Ok(Local {
            working_directory: env::current_dir()?.to_string_lossy().into(),
        })
    }
}

impl FileOperations for Local {
    fn get_files(&mut self) -> Result<Vec<String>> {
        log::info!("Going to: {}", self.get_working_directory());
        let paths = fs::read_dir(self.get_working_directory())?;
        let mut files = Vec::new();

        for path in paths {
            let path_value = path?.path();
            if let Some(filename) = path_value.file_name() {
                if !path_value.is_dir() {
                    files.push(filename.to_string_lossy().into());
                } else {
                    files.push(format!("{}/", filename.to_string_lossy()));
                }
            }
        }

        Ok(files)
    }

    // TODO: Make these functions work on Non-unix systems e.g Windows
    fn change_directory_rel(&mut self, path: &str) {
        let concat_dir = &format!("{}/{}", self.working_directory, path);
        if self.is_directory(concat_dir.to_string()) {
            self.working_directory = concat_dir.to_string();
        }
    }

    fn level_up_files(&mut self) -> Result<Vec<String>> {
        let mut splited_path = self.working_directory.split("/").collect::<Vec<&str>>();
        if splited_path.len() > 2 {
            splited_path.remove(splited_path.len() - 1);
            log::info!("Len: {}", splited_path.len());
            self.working_directory = splited_path.join("/");
        } else {
            self.working_directory = "/".to_string();
        }
        self.get_files()
    }


    fn is_directory(&self, path: String) -> bool {
        let md = fs::metadata(path);
        match md {
            Ok(metadata) => metadata.is_dir(),
            _ => false,
        }
    }

    fn get_working_directory(&self) -> &str {
        self.working_directory.as_str()
    }
}
