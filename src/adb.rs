use std::process::Command;

use anyhow::Context;
use anyhow::Result;

use crate::file_operations::FileOperations;

#[derive(Clone)]
pub struct Device {
    name: String,
    authorized: bool,
    working_directory: String,
}

impl Device {
    pub fn new(adb_output: &str) -> Option<Self> {
        let device_info = adb_output.split("\t").collect::<Vec<&str>>();

        if device_info.len() == 2 {
            Some(Self {
                name: device_info[0].to_string(),
                authorized: device_info[1] == "device",
                working_directory: "/".to_string(),
            })
        } else {
            None
        }
    }
}

impl FileOperations for Device {
    fn get_files(&mut self) -> Result<Vec<String>> {
        let mut adb = Command::new("adb");
        let mut files = Vec::new();

        adb.arg("-s");
        adb.arg(&self.name);
        adb.args(vec!["shell", "ls", &self.working_directory, "-p"]);

        let adb_output_full = adb
            .output()
            .with_context(|| format!("Failed to create adb process"))?;

        for file in String::from_utf8_lossy(&adb_output_full.stdout)
            .to_string()
            .split("\n")
        {
            files.push(file.to_string());
        }
        Ok(files)
    }

    fn change_directory_rel(&mut self, path: &str) {
        self.working_directory = format!("{}{}", self.working_directory, path);
    }

    fn level_up_files(&mut self) -> Result<Vec<String>> {
        let mut splited_path = self.working_directory.split("/").collect::<Vec<&str>>();
        splited_path.remove(splited_path.len() - 1);
        self.working_directory = splited_path.join("/");
        self.get_files()
    }

    fn is_directory(&self, path: String) -> bool {
        path.ends_with("/")
    }

    fn get_working_directory(&self) -> &str {
        self.working_directory.as_str()
    }
}

pub struct Adb {
    pub devices: Vec<Device>,
}

impl Adb {
    pub fn new() -> Result<Self> {
        let mut adb = Command::new("adb");
        adb.arg("start-server");

        adb.status().with_context(|| {
            format!("Failed to start adb server, check for adb installation in system")
        })?;

        Ok(Self {
            devices: Vec::new(),
        })
    }

    pub fn populate_devices(&mut self) -> Result<()> {
        let mut adb = Command::new("adb");
        adb.arg("devices");

        let adb_output_full = adb
            .output()
            .with_context(|| format!("Failed to create adb process"))?;

        for device_str in String::from_utf8_lossy(&adb_output_full.stdout)
            .to_string()
            .split("\n")
            .skip(1)
        {
            if let Some(device) = Device::new(device_str) {
                self.devices.push(device);
            }
        }

        Ok(())
    }
}
