use crate::{file_operations::FileOperations, layout::Layout, listbox::ListBox};
use anyhow::Result;
use console_engine::{ConsoleEngine, KeyCode};

pub struct DeviceFilelist<T: FileOperations> {
    pub listbox: ListBox,
    pub device_files: T,
}

impl<T: FileOperations> DeviceFilelist<T>
where
    T: FileOperations,
{
    pub fn new(layout: &Layout, mut device_files: T) -> Result<Self> {
        let mut ls = ListBox::new(layout, false);
        ls.set_content(device_files.get_files()?);

        Ok(Self {
            device_files,
            listbox: ls,
        })
    }

    pub fn handle_listbox(&mut self, engine: &ConsoleEngine) -> Result<()> {
        self.listbox.handle_events(engine);
        if self.listbox.focused {
            if engine.is_key_pressed(KeyCode::Enter) {
                self.device_files
                    .change_directory_rel(self.listbox.get_selected_str().as_str());
                let files = self.device_files.get_files()?;
                self.listbox.set_content(files);
            }
        }

        if engine.is_key_pressed(KeyCode::Tab) {
            self.listbox.focused = !self.listbox.focused;
        }

        if self.listbox.focused && engine.is_key_pressed(KeyCode::Backspace) {
            let files = self.device_files.level_up_files()?;
            self.listbox.set_content(files);
        }
        Ok(())
    }

    pub fn update_filelist(&mut self) -> Result<()> {
        let files = self.device_files.get_files()?;
        self.listbox.set_content(files);
        Ok(())
    }
}
