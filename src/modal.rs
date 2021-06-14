use console_engine::{pixel, rect_style, screen::Screen, ConsoleEngine, KeyCode};

fn draw_buttons(screen: &mut Screen, buttons: Vec<&str>, selected: usize) {
    for (idx, button) in buttons.iter().enumerate() {
        if idx == selected {
            screen.print_fbg(
                idx as i32 * (button.chars().count() + 1) as i32 + 1,
                screen.get_height() as i32 - 2,
                button,
                console_engine::Color::Black,
                console_engine::Color::White,
            );
        } else {
            screen.print(
                idx as i32 * (button.chars().count() + 1) as i32 + 1,
                screen.get_height() as i32 - 2,
                button,
            );
        }
    }
}

pub fn modal(engine: &mut ConsoleEngine, message: String, buttons: Vec<&str>) -> String {
    let w = message.chars().count() as u32 + 2;
    let h = engine.get_height() / 8;

    let x = (engine.get_width() as i32 - message.chars().count() as i32) / 2;
    let y = engine.get_height() as i32 / 2;
    let mut screen = Screen::new_fill(w, h, pixel::pxl(' '));
    let mut selected = 0;

    screen.rect_border(
        0,
        0,
        screen.get_width() as i32 - 1,
        screen.get_height() as i32 - 1,
        rect_style::BorderStyle::new_light(),
    );
    screen.print(1, 2, message.as_str());
    draw_buttons(&mut screen, buttons.clone(), selected);

    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen
        engine.check_resize();
        draw_buttons(&mut screen, buttons.clone(), selected);
        engine.print_screen(x, y, &screen);
        engine.draw();

        if engine.is_key_pressed(KeyCode::Enter) {
            return buttons[selected].to_string();
        }

        if engine.is_key_pressed(KeyCode::Left) || engine.is_key_pressed(KeyCode::BackTab) {
            selected = selected.saturating_sub(1);
        }

        if engine.is_key_pressed(KeyCode::Right) || engine.is_key_pressed(KeyCode::Tab) {
            selected = (selected + 1).clamp(0, buttons.len());
        }
    }
}
