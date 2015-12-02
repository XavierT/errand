extern crate ncurses;

use ncurses::*;

/// Main function
fn main(){

  let locale_conf = LcCategory::all;

  // Necessary for utf8 supper
  // before initscr
  ncurses::setlocale(locale_conf,"");

  /* Start ncurses. */
  ncurses::initscr();

  /* Print to the back buffer. */
  ncurses::printw("Hello, world!");

  /* Print some unicode(Chinese) string. */
  ncurses::printw("\n我喜欢中国茶");

  /* Update the screen. */
  ncurses::refresh();

  /* Wait for a key press. */
  ncurses::getch();

  /* Terminate ncurses. */
  ncurses::endwin();
}
