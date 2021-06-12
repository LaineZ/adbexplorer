use console_engine::{pixel, screen::Screen, Color, ConsoleEngine, KeyCode};

use crate::layout::Layout;

pub trait ListBoxItem {
    fn get_entry(&self) -> String;
    fn bg_color(&self) -> Color;
    fn fg_color(&self) -> Color;
}

#[derive(Clone)]
/// A listbox user interface
pub struct ListBox {
    /// Listbox items
    pub display: Vec<String>,
    /// Current displaying page
    pub page: usize,
    /// Current on-page cursor position. you can safely read/write this value
    pub position: usize,
    screen: Screen,
    /// In focus this listbox?
    pub focused: bool,
    x: i32,
    y: i32,
}

impl ListBox {
    /// Creates listbox
    pub fn new(layout: &Layout, focused: bool) -> Self {
        Self {
            display: Vec::new(),
            x: layout.x,
            y: layout.y,
            page: 0,
            position: 0,
            screen: Screen::new_fill(layout.w as u32, layout.h as u32, pixel::pxl(' ')),
            focused,
        }
    }

    /// Gets listbox page count
    pub fn get_page_count(&mut self) -> usize {
        self.display
            .chunks((self.screen.get_height() - 2) as usize)
            .len()
    }

    /// Scrolls listbox down by 1 item. if reaches end of current page - switches to another
    pub fn scroll_down(&mut self) {
        if self.position < self.screen.get_height() as usize - 2 {
            self.position += 1;
        } else {
            self.switch_page_up();
        }
    }

    /// Scrolls listbox up by 1 item. if reaches start of current page - switches to another
    pub fn scroll_up(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        } else {
            self.switch_page_down();
            self.position = self.screen.get_height() as usize - 2;
        }
    }

    /// Scrolls listbox up by 1 page
    pub fn switch_page_up(&mut self) {
        if self.page < self.get_page_count() - 1 {
            self.page += 1;
            self.position = 0;
        }
        self.screen.clear();
    }

    /// Scrolls listbox down by 1 page
    pub fn switch_page_down(&mut self) {
        if self.page > 0 {
            self.page -= 1;
        }
        self.screen.clear();
    }


    pub fn set_content(&mut self, content: Vec<String>) {
        self.screen.clear();
        self.display = content;
        self.page = 0;
        self.position = 0;
    }

    /// Gets selected listbox index
    pub fn get_selected_idx(&mut self) -> usize {
        let pos = self.position + (self.page * self.screen.get_height() as usize);
        pos.checked_sub(2).unwrap_or(0)
    }

    /// Removes listbox items by value
    pub fn remove(&mut self, value: String) {
        self.display.retain(|x| x == &value);
    }

    /// Gets current selected String in listbox
    pub fn get_selected_str(&mut self) -> String {
        self.display[self.position].clone()
    }

    /// Sets listbox position and sets the needed page
    pub fn sel_idx_glob(&mut self, pos: usize) -> usize {
        pos + (self.page * self.screen.get_height() as usize)
    }

    /// Resizes listbox to specified dimensions
    pub fn resize(&mut self, layout: &Layout) {
        self.position = 0;
        self.screen.clear();
        self.screen.resize(layout.w as u32, layout.h as u32);
        self.x = layout.x;
        self.y = layout.y;
    }

    pub fn handle_events(&mut self, engine: &ConsoleEngine) {
        if self.focused {
            if engine.is_key_held(KeyCode::Down) {
                self.scroll_down();
            }

            if engine.is_key_held(KeyCode::Up) {
                self.scroll_up();
            }
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Draws listbox. For more stability recommended to use in print_screen function
    pub fn draw(&mut self) -> &Screen {
        let splited_pags = self.display.chunks(self.screen.get_height() as usize);

        let mut fg_color = Color::Reset;

        if !self.focused {
            fg_color = Color::DarkGrey;
        }

        for (i, v) in &mut splited_pags.into_iter().enumerate() {
            if i == self.page {
                for (index, page) in v.into_iter().enumerate() {
                    if index == self.position && self.focused {
                        self.screen
                            .print_fbg(0, index as i32, page, fg_color, Color::White)
                    } else {
                        self.screen
                            .print_fbg(0, index as i32, page, fg_color, Color::Reset);
                    }
                }
            }
        }

        for y in 0..self.screen.get_height() {
            self.screen
                .print(self.screen.get_width() as i32 - 1, y as i32, "|");
        }

        &self.screen
    }
}
