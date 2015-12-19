extern crate ncurses;

use self::ncurses::*;

// Window are actually implemented has ncurcesw subwin
// of the stdscr
pub struct Window {
    start_x: i32,
    start_y: i32,

    size_x: i32,
    size_y: i32,

    handle: WINDOW, // ncurses subwin handle

    renderer: Box<Render>,
}

impl Window {
    pub fn new(start_y: i32,
               start_x: i32,
               size_y: i32,
               size_x: i32,
               renderer: Box<Render>)
               -> Window {
        Window {
            start_x: start_x,
            start_y: start_y,
            size_x: size_x,
            size_y: size_y,
            renderer: renderer,
            handle: stdscr, // hack init the subwin to the whole window
        }
    }

    pub fn init(&mut self) {

        self.handle = ncurses::subwin(stdscr,
                                      self.start_y + self.size_y,
                                      self.start_x + self.size_x,
                                      self.start_y,
                                      self.start_x);
        ncurses::box_(self.handle, 0, 0);
        // ncurses::waddstr(self.handle, &self.name as str);
    }

    pub fn write(&self, text: String) {}

    pub fn refresh(&self) {}
}

pub trait Render {
    fn render();
}

struct StoryWin;
struct MapWin;
struct StatusWin;

impl Render for StoryWin {
    fn render() {
        println!("I am story!");
    }
}


impl Render for MapWin {
    fn render() {
        println!("I'm map!");
    }
}

impl Render for StatusWin {
    fn render() {
        println!("I'm program");
    }
}
