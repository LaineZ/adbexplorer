use console_engine::{pixel, rect_style, screen::Screen, Color, ConsoleEngine, KeyCode};

fn draw_buttons(screen: &mut Screen, buttons: Vec<&str>, selected: usize) {
    for (idx, button) in buttons.iter().enumerate() {
        let coord_x = idx as i32 * (button.chars().count() + 1) as i32 + 2;

        if idx == selected {
            screen.print_fbg(
                coord_x,
                screen.get_height() as i32 - 2,
                button,
                Color::Black,
                Color::White,
            );
        } else {
            screen.print(coord_x, screen.get_height() as i32 - 2, button);
        }

        screen.print_fbg(
            coord_x - 1,
            screen.get_height() as i32 - 2,
            (idx + 1).to_string().as_str(),
            Color::White,
            Color::DarkBlue,
        )
    }
}

fn recompute_sizes(ew: u32, eh: u32, count: u32) -> (i32, i32, u32, u32) {
    let w = (count + 2).clamp(10, ew);
    let h = (eh / 4).clamp(5, eh as u32);
    let mut x = 0;
    let y = eh as i32 / 4;

    if w < ew {
        x = (ew as i32 - count as i32) / 2;
    }

    (x, y, w, h)
}

pub fn modal(engine: &mut ConsoleEngine, message: String, buttons: Vec<&str>) -> String {
    let (_, _, w, h) = recompute_sizes(engine.get_width(), engine.get_height(), message.chars().count() as u32);
    let mut screen = Screen::new_fill(w, h, pixel::pxl(' '));
    let mut selected = 0;

    screen.rect_border(
        0,
        0,
        screen.get_width() as i32 - 1,
        screen.get_height() as i32 - 1,
        rect_style::BorderStyle::new_light(),
    );
    screen.print(1, 1, textwrap::fill(&message, w as usize).as_str());
    draw_buttons(&mut screen, buttons.clone(), selected);

    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen
        engine.check_resize();
        draw_buttons(&mut screen, buttons.clone(), selected);
        let (x, y, _, _) = recompute_sizes(engine.get_width(), engine.get_height(), message.chars().count() as u32);
        engine.print_screen(x, y, &screen);
        engine.draw();

        if engine.is_key_pressed(KeyCode::Enter) {
            return buttons[selected].to_string();
        }

        if engine.is_key_pressed(KeyCode::Left) {
            selected = selected.saturating_sub(1);
        }

        if engine.is_key_pressed(KeyCode::Right) {
            selected = (selected + 1).clamp(0, buttons.len() - 1);
        }

        for i in 0..buttons.len() {
            if engine.is_key_pressed(KeyCode::Char(
                char::from_digit(i as u32 + 1, 10).unwrap_or('1'),
            )) {
                return buttons[i].to_string();
            }
        }

        if let Some((new_w, new_h)) = engine.get_resize() {
            let (_, _, w, h) = recompute_sizes(new_w as u32, new_h as u32, message.chars().count() as u32);
            screen.resize(w as u32, h as u32);
            screen.clear();
            screen.rect_border(
                0,
                0,
                screen.get_width() as i32 - 1,
                screen.get_height() as i32 - 1,
                rect_style::BorderStyle::new_light(),
            );
            screen.print(1, 1, textwrap::fill(&message, w as usize).as_str());
            draw_buttons(&mut screen, buttons.clone(), selected);
        }
    }
}
