#![allow(dead_code)]

extern crate ncurses;
extern crate time;
#[macro_use]
extern crate log;

pub mod ui;
pub mod util;

use ncurses::*;
use ui::window::Window;
use ui::input::InputHandler;

use util::file_logger::SimpleFileLogger;

/// Main function
fn main() {

    // Might need to actually check the return value at some point
    let _ = SimpleFileLogger::init();

    info!("Starting...");

    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    let locale_conf = LcCategory::all;

    // Necessary for utf8 support
    // before initscr
    ncurses::setlocale(locale_conf, "");

    // Start ncurses.
    ncurses::initscr();
    ncurses::cbreak();
    // Enable some additional keys, like arrow keys, or F1
    unsafe {
        ncurses::keypad(stdscr, true);
    }
    // Do not display typed character in terminal
    ncurses::noecho();

    unsafe {
        ncurses::getmaxyx(stdscr, &mut max_y, &mut max_x);
    }

    // Create 3 windows
    // width of the status column
    let status_width = 20;
    let log_height = 5;

    let mut status = Window::new(0, 0, max_y, status_width);
    status.init();

    let mut map = Window::new(0, status_width, max_y - log_height, max_x - status_width);
    map.init();

    let mut log = Window::new(max_y - log_height,
                              status_width,
                              log_height,
                              max_x - status_width);
    log.init();

    // Print to the map windows.
    map.write("Map".to_string());
    log.write("log".to_string());

    // Update the screen.
    map.refresh();
    status.refresh();
    log.refresh();


    loop {
        // Wait for a key press.
        let c: i32 = ncurses::getch();
        let ch: char = std::char::from_u32(c as u32).unwrap();

        let input = InputHandler::new();

        if ch == 'q' {
            break;
        } else {
            input.process_key(ch);
        }

        map.refresh();
        status.refresh();
        log.refresh();
    }

    // Terminate ncurses.
    map.delwin();
    status.delwin();
    log.delwin();

    ncurses::endwin();
}
