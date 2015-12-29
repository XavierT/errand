extern crate ncurses;

pub mod gui;

use ncurses::*;
use gui::window::{Window,StoryWin};

/// Main function
fn main() {

    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    let locale_conf = LcCategory::all;

    // Necessary for utf8 support
    // before initscr
    ncurses::setlocale(locale_conf, "");
    ncurses::cbreak();

    // Start ncurses.
    ncurses::initscr();

    ncurses::getmaxyx(stdscr, &mut max_y, &mut max_x);

    // Create 3 windows
    // width of the story column
    let story_width = 30;
    let status_height = 3;

    //let story = subwin(stdscr, max_y, story_width, 0, 0);
    let mut story = Window::new(0,0,
                           max_y,story_width,
                           Box::new(StoryWin));
    story.init();
    let map = subwin(stdscr,
                     max_y - status_height,
                     max_x - story_width,
                     0,
                     story_width);
    let status = subwin(stdscr,
                        status_height,
                        max_x - story_width,
                        max_y - status_height,
                        story_width);
    ncurses::box_(map, 0, 0);
    //ncurses::box_(story, 0, 0);
    ncurses::box_(status,0,0);

    // Print to the map windows.
    ncurses::waddstr(map, "Map");

    // Print to the map windows.
    //ncurses::waddstr(story, "Story");
    ncurses::waddstr(status, "Status");

    // Update the screen.
    ncurses::wrefresh(map);
    //ncurses::wrefresh(story);
    ncurses::wrefresh(status);

    let mut x = 1;
    let mut y = 1;

    loop {
        // Wait for a key press.
        let ch = ncurses::getch();

        if ch == 'q' as i32 {
            break;
        } else {
            ncurses::mvwaddch(map, y, x, ch as u64);
            x += 1;
        }

        // ncurses::wrefresh(stdscr);
        ncurses::wrefresh(map);
        // ncurses::wrefresh(story);
    }

    // Terminate ncurses.
    ncurses::delwin(map);
    //ncurses::delwin(story);
    story.delwin();
    ncurses::delwin(status);
    ncurses::endwin();

}
