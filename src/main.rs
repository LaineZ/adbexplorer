use std::vec;

use adb::Device;
use anyhow::anyhow;
use anyhow::Result;
use console_engine::ConsoleEngine;
use device_filelist::DeviceFilelist;
use file_operations::{FileOperations, Local};
use flexi_logger::FileSpec;
use generational_arena::Index;
use layout::{Direction, LayoutEngine, LayoutSize, LayoutStyle};
use modal::modal;

use crate::adb::Adb;

mod adb;
mod bottom_bar;
mod device_filelist;
mod file_operations;
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

fn main() {
    flexi_logger::Logger::try_with_str("warn, adbexplorer=debug")
        .unwrap()
        .log_to_file(FileSpec::default())
        .format_for_files(flexi_logger::with_thread)
        .start()
        .unwrap();

    let mut engine = console_engine::ConsoleEngine::init_fill(30)
        .expect("Unable to create console engine instance");
    loop {
        if let Err(err) = main_inner(&mut engine) {
            let result = modal(
                &mut engine,
                format!("Error: {}", err.to_string().replace("\n", " ")),
                vec!["Exit", "Restart"],
            );

            if result.as_str() == "Exit" {
                break;
            }
        }
    }
}

fn main_inner(engine: &mut ConsoleEngine) -> Result<()> {
    // UI SETUP
    let (cols, rows) = (engine.get_width() as u16, engine.get_height() as u16);
    let mut main_layout = LayoutEngine::new();
    let (left_idx, right_idx) = resize_layout(&mut main_layout, cols, rows);
    let left_l = main_layout.get_layout(left_idx).unwrap();
    let right_l = main_layout.get_layout(right_idx).unwrap();

    // FILE LIST SETUP
    let mut adb = Adb::new()?;
    let local = Local::new()?;
    adb.populate_devices()?;
    if adb.devices.is_empty() {
        return Err(anyhow!(
            "No adb devices present in system. Check your ADB connection on phone"
        ));
    }

    let device = adb.devices[0].clone();
    // SETTING PANES
    let mut device_pane = DeviceFilelist::new(&left_l, device)?;
    let mut local_pane = DeviceFilelist::new(&right_l, local)?;
    let mut bottom_bar = bottom_bar::StateBar::new(&engine);

    device_pane.listbox.focused = true;

    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen
        engine.check_resize();

        engine.print_screen(
            device_pane.listbox.get_position().0,
            device_pane.listbox.get_position().1,
            device_pane.listbox.draw(),
        );
        engine.print_screen(
            local_pane.listbox.get_position().0,
            local_pane.listbox.get_position().1,
            local_pane.listbox.draw(),
        );

        engine.print_screen(0, bottom_bar.y as i32, bottom_bar.draw());

        device_pane.handle_listbox(&engine)?;
        local_pane.handle_listbox(&engine)?;

        if device_pane.listbox.focused {
            bottom_bar.set_text(device_pane.device_files.get_working_directory());
        } else {
            bottom_bar.set_text(local_pane.device_files.get_working_directory());
        }

        engine.draw();

        if engine.is_key_held(console_engine::KeyCode::Esc) {
            std::process::exit(0);
        }

        if let Some((w, h)) = engine.get_resize() {
            log::info!("Resized to {}x{}", w, h);
            let mut main_layout = LayoutEngine::new();

            let (left_idx, right_idx) = resize_layout(&mut main_layout, w, h);

            let left_l = main_layout.get_layout(left_idx).unwrap();
            let right_l = main_layout.get_layout(right_idx).unwrap();

            device_pane.listbox.resize(left_l);
            local_pane.listbox.resize(right_l);
            bottom_bar.resize(w, h);

            log::info!("Layout: {:#?}", left_l);
        }
    }
}
