extern crate ncurses;

use self::ncurses::*;

// Window are actually implemented has ncurcesw subwin
// of the stdscr
pub struct Window
{
    start_x : i32,
    start_y : i32,

    size_x : i32,
    size_y : i32,

    name : String,
    handle : WINDOW,  // ncurses subwin handle

}

impl Window
{
    pub fn new(name : String, start_y : i32, start_x : i32, size_y : i32, size_x : i32) -> Window{
        Window {
            name: name,
            start_x : start_x,
            start_y : start_y,
            size_x : size_x,
            size_y : size_y,
            handle : stdscr // hack init the subwin to the whole window
        }
    }

    pub fn init(&mut self) {

        self.handle= ncurses::subwin(stdscr,
                                     self.start_y+self.size_y,
                                     self.start_x+self.size_x,
                                     self.start_y,
                                     self.start_x);
        ncurses::box_(self.handle,0,0);
        //ncurses::waddstr(self.handle, &self.name as str);
    }

    pub fn write(&self, text : String) {

    }

    pub fn refresh(&self) {

    }


}




