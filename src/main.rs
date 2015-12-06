extern crate ncurses;

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

  // Create 3 windows
  let width = 30;

  //let story = newwin(max_y - score, max_x, 0,0);
  //let map = newwin(score, max_x, max_y - score, 0);
  let story = newwin(max_y, width, 0,0);
  let map = newwin(max_y, max_x-width, 0, width);
  ncurses::box_(map,0,0);
  ncurses::box_(story,0,0);

  /* Print to the map windows. */
  ncurses::waddstr(map,"Map");

  /* Print to the map windows. */
  ncurses::waddstr(story, "Story");

  /* Update the screen. */
  wrefresh(stdscr);  //--> still need to refresh the whole screen

  ncurses::wrefresh(map);
  ncurses::wrefresh(story);

  /* Wait for a key press. */
  ncurses::getch();

  /* Terminate ncurses. */
  delwin(map);
  delwin(story);
  ncurses::endwin();
}
