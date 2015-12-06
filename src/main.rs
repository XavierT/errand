extern crate ncurses;

use std::fmt;

use ncurses::*;

/// Main function
fn main(){

  let mut max_x : i32 = 0;
  let mut max_y : i32 = 0;

  let locale_conf = LcCategory::all;

  // Necessary for utf8 support
  // before initscr
  ncurses::setlocale(locale_conf,"");

  /* Start ncurses. */
  ncurses::initscr();

  ncurses::getmaxyx(stdscr, &mut max_y, &mut max_x);

  let y = max_y / 2;
  let x = max_x / 2;

  ncurses::wmove(stdscr,y, x);

  /* Print to the back buffer. */
  ncurses::printw("Hello, world!\n");
  let s: String = format!("This terminal is of size y:{} x:{}",max_y, max_x);
  let s_slice:&str = &s[..];

  ncurses::mvprintw(y+1,x,s_slice);
  /* Print some unicode(Chinese) string. */
  ncurses::printw("\n我喜欢中国茶");

  /* Update the screen. */
  ncurses::refresh();

  /* Wait for a key press. */
  ncurses::getch();

  /* Terminate ncurses. */
  ncurses::endwin();
}
