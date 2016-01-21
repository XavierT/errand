extern crate ncurses;

pub mod ui;

use ncurses::*;
use ui::window::{Window, StatusWin, MapWin, LogWin};

/// Main function
fn main() {

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
    ncurses::keypad(stdscr, true);
    // Do not display typed character in terminal
    ncurses::noecho();

    ncurses::getmaxyx(stdscr, &mut max_y, &mut max_x);

    // Create 3 windows
    // width of the status column
    let status_width = 20;
    let log_height = 5;

    let mut status = Window::new(0, 0, max_y, status_width, Box::new(StatusWin));
    status.init();

    let mut map = Window::new(0,
                              status_width,
                              max_y - log_height,
                              max_x - status_width,
                              Box::new(MapWin));
    map.init();

    let mut log = Window::new(max_y - log_height,
                                 status_width,
                                 log_height,
                                 max_x - status_width,
                                 Box::new(LogWin));
    log.init();

    // Print to the map windows.
    map.write("Map".to_string());
    log.write("log".to_string());

    // Update the screen.
    map.refresh();
    status.refresh();
    log.refresh();

    let mut x = 1;
    let mut y = 1;

    loop {
        // Wait for a key press.
        let ch = ncurses::getch();

        if ch == 'q' as i32 {
            break;
        } else {
            // ncurses::mvwaddch(map, y, x, ch as u64);
            status.mvaddch(y, x, ch as u64);
            log.mvaddch(y, x, ch as u64);
            map.mvaddch(y, x, ch as u64);
            x += 1;
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
