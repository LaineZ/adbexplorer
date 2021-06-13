use console_engine::{Color, ConsoleEngine, pixel, screen::Screen};
use Into;


/// Bars interface
#[derive(Clone)]
pub struct StateBar {
    pub message: String,
    /// Is error occured? Set header bar background color to red
    error: bool,
    /// Screen struct
    screen: Screen,
    /// Y position of the bar
    pub y: u32,
}

impl StateBar {
    /// Creates a state bars
    pub fn new(engine: &ConsoleEngine) -> Self {
        Self {
            message: String::new(),
            error: false,
            screen: Screen::new(engine.get_width().into(), 1),
            y: engine.get_height() as u32 - 1,
        }
    }

    /// Draws a bar and its widgets
    pub fn draw(&mut self) -> &Screen {
        self.screen.fill(pixel::pxl_bg(' ', Color::Blue));
        self.screen.print_fbg(0, 0, self.message.as_str(), Color::Reset, Color::Blue);
        &self.screen
    }

    pub fn set_text<T: AsRef<str>>(&mut self, item: T) {
        self.message = item.as_ref().to_string();
    }

    /// Resizes bar
    pub fn resize(&mut self, w: u16, h: u16) {
        self.screen.clear();
        self.screen.resize(w as u32, 1);
        self.y = h as u32;
    }
}