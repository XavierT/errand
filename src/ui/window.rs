extern crate ncurses;

use self::ncurses::*;

// Window are actually implemented has ncurcesw subwin
// of the stdscr
pub struct Window {
    start_y: i32,
    start_x: i32,

    size_y: i32,
    size_x: i32,

    handle: WINDOW, // ncurses subwin handle
}

impl Window {
    pub fn new(start_y: i32, start_x: i32, size_y: i32, size_x: i32) -> Window {
        Window {
            start_y: start_y,
            start_x: start_x,
            size_y: size_y,
            size_x: size_x,
            handle: stdscr, // hack init the subwin to the whole window
        }
    }

    pub fn init(&mut self) {

        self.handle = ncurses::subwin(stdscr, self.size_y, self.size_x, self.start_y, self.start_x);
        ncurses::box_(self.handle, 0, 0);
    }

    pub fn write(&self, text: String) {
        ncurses::waddstr(self.handle, text.as_ref());
    }

    pub fn mvaddch(&self, y: i32, x: i32, ch: u64) {
        ncurses::mvwaddch(self.handle, y, x, ch);
    }

    pub fn refresh(&self) {
        ncurses::wrefresh(self.handle);
    }

    pub fn delwin(&self) {
        ncurses::delwin(self.handle);
    }
}
