use console_engine::{ConsoleEngine, KeyCode, pixel, rect_style, screen::Screen};

pub struct Modal {
    screen: Screen,
    buttons: Vec<String>,
    message: String,
    x: i32,
    y: i32,
}

impl Modal {
    pub fn new(engine: &ConsoleEngine, message: String, buttons: Vec<String>) -> Self {
        let w = message.chars().count() as u32 + 2;
        let h = engine.get_height() / 8;
        Self {
            screen: Screen::new_fill(w, h, pixel::pxl(' ')),
            buttons,
            message,
            x: (engine.get_width() as i32) / 2 - w as i32,
            y: engine.get_height() as i32 / 2,
        }
    }

    fn draw(&mut self) {
        self.screen.rect_border(
            0,
            0,
            self.screen.get_width() as i32 - 1,
            self.screen.get_height() as i32 - 1,
            rect_style::BorderStyle::new_light(),
        );
        self.screen.print(1, 2, &self.message);

        for (idx, button) in self.buttons.iter().enumerate() {
            self.screen.print_fbg(idx as i32 + button.len() as i32 * 2, self.screen.get_height() as i32 - 2, button, console_engine::Color::Black, console_engine::Color::White);
        }
    }

    pub fn show_window(&mut self, engine: &mut ConsoleEngine) {
        self.draw();
        engine.print_screen(self.x, self.y, &self.screen);
    }
}
