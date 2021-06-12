use std::vec;

use anyhow::Result;
use console_engine::KeyCode;
use flexi_logger::FileSpec;
use generational_arena::Index;
use layout::{Direction, LayoutEngine, LayoutSize, LayoutStyle};
use listbox::ListBox;

use crate::{adb::Adb, modal::Modal};

mod adb;
mod layout;
mod listbox;
mod modal;

fn resize_layout(main_layout: &mut LayoutEngine, w: u16, h: u16) -> (Index, Index) {
    let left_idx = main_layout.new_node(LayoutStyle::default(), vec![]);
    let right_idx = main_layout.new_node(LayoutStyle::default(), vec![]);

    let root = main_layout.new_node(
        LayoutStyle {
            direction: Direction::LeftRight,
        },
        vec![left_idx, right_idx],
    );

    main_layout
        .compute_layout(root, LayoutSize::new(w as i32, h as i32 - 1))
        .unwrap();

    return (left_idx, right_idx);
}

fn main() -> Result<()> {
    let mut engine = console_engine::ConsoleEngine::init_fill(30)?;
    let (cols, rows) = (engine.get_width() as u16, engine.get_height() as u16);

    let mut main_layout = LayoutEngine::new();

    let (left_idx, right_idx) = resize_layout(&mut main_layout, cols, rows);
    let left_l = main_layout.get_layout(left_idx).unwrap();
    let right_l = main_layout.get_layout(right_idx).unwrap();

    // listboxes
    let mut left = ListBox::new(left_l, true);
    let mut right = ListBox::new(right_l, false);

    // adb
    let mut adb = Adb::new()?;
    adb.populate_devices()?;

    let mut files = adb.devices[0].get_files()?;
    left.set_content(files);
    right.set_content(vec!["Yes".to_string(), "No".to_string()]);

    flexi_logger::Logger::try_with_str("warn, adbexplorer=debug")
        .unwrap()
        .log_to_file(FileSpec::default())
        .format_for_files(flexi_logger::with_thread)
        .start()
        .unwrap();

    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen
        engine.check_resize();

        engine.print_screen(left.get_position().0, left.get_position().1, left.draw());
        engine.print_screen(right.get_position().0, right.get_position().1, right.draw());
        left.handle_events(&engine);
        right.handle_events(&engine);

        // TODO: same for right
        if left.focused && engine.is_key_pressed(KeyCode::Enter) {
            files = adb.devices[0].get_files()?;
            left.set_content(files);
        }

        if left.focused && engine.is_key_pressed(KeyCode::Enter) {
            adb.devices[0].change_directory_rel(left.get_selected_str().as_str());
            files = adb.devices[0].get_files()?;
            left.set_content(files);
        }

        if left.focused && engine.is_key_pressed(KeyCode::Backspace) {
            files = adb.devices[0].level_up_files()?;
            left.set_content(files);
        }
        
        engine.draw();

        if engine.is_key_held(console_engine::KeyCode::Esc) {
            break;
        }

        if let Some((w, h)) = engine.get_resize() {
            log::info!("Resized to {}x{}", w, h);
            let mut main_layout = LayoutEngine::new();

            let (left_idx, right_idx) = resize_layout(&mut main_layout, w, h);

            let left_l = main_layout.get_layout(left_idx).unwrap();
            let right_l = main_layout.get_layout(right_idx).unwrap();

            left.resize(left_l);
            right.resize(right_l);

            log::info!("Layout: {:#?}", left_l);
        }
    }

    Ok(())
}
