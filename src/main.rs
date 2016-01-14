extern crate ncurses;

pub mod ui;

use ncurses::*;
use ui::window::{Window, StoryWin, MapWin, StatusWin};

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
    // Do not display typed character in terminal
    ncurses::noecho();

    ncurses::getmaxyx(stdscr, &mut max_y, &mut max_x);

    // Create 3 windows
    // width of the story column
    let story_width = 30;
    let status_height = 3;

    // let story = subwin(stdscr, max_y, story_width, 0, 0);
    let mut story = Window::new(0, 0, max_y, story_width, Box::new(StoryWin));
    story.init();

    let mut map = Window::new(0,
                              story_width,
                              max_y - status_height,
                              max_x - story_width,
                              Box::new(MapWin));
    map.init();

    let mut status = Window::new(max_y - status_height,
                                 story_width,
                                 status_height,
                                 max_x - story_width,
                                 Box::new(StatusWin));
    status.init();

    // Print to the map windows.
    map.write("Map".to_string());
    status.write("Status".to_string());

    // Update the screen.
    map.refresh();
    story.refresh();
    status.refresh();

    let mut x = 1;
    let mut y = 1;

    loop {
        // Wait for a key press.
        let ch = ncurses::getch();

        if ch == 'q' as i32 {
            break;
        } else {
            // ncurses::mvwaddch(map, y, x, ch as u64);
            story.mvaddch(y, x, ch as u64);
            status.mvaddch(y, x, ch as u64);
            map.mvaddch(y, x, ch as u64);
            x += 1;
        }

        map.refresh();
        story.refresh();
        status.refresh();
    }

    // Terminate ncurses.
    map.delwin();
    story.delwin();
    status.delwin();

    ncurses::endwin();
}
