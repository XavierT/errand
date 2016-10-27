#![allow(unused_variables)]

/// Mode of input similar to Vim input mode.
///
/// In each mode the key will have a different meaning
#[derive(Debug)]
pub enum InputMode {
    /// Normal game mode
    Normal,
    /// Move cursor freely around to identify things
    Explore,
    /// Menu
    Menu,
    /// Yes or no question
    YesOrNo,
    /// Help files
    Help,
}

pub struct InputHandler {
    mode: InputMode,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler { mode: InputMode::Normal }
    }

    pub fn change_mode(&mut self, new_mode: InputMode) {
        self.mode = new_mode;
    }

    pub fn process_key(&self, key: char) {
        match self.mode {
            InputMode::Normal => self.process_normal(key),
            InputMode::Explore => self.process_explore(key),
            InputMode::Menu => self.process_menu(key),
            InputMode::YesOrNo => self.process_yesorno(key),
            InputMode::Help => self.process_help(key),
        }
    }

    fn process_normal(&self, key: char) {
        match key {
            'h' => info!("west"),
            'j' => info!("south"),
            'k' => info!("north"),
            'l' => info!("left"),
            'y' => info!("left"),
            'u' => info!("left"),
            'b' => info!("left"),
            'n' => info!("left"),
            _ => warn!("unknown command in {:?} mode : {}", self.mode, key),
        }
    }

    fn process_explore(&self, key: char) {
        unimplemented!();
    }

    fn process_menu(&self, key: char) {
        unimplemented!();
    }

    fn process_yesorno(&self, key: char) {
        unimplemented!();
    }

    fn process_help(&self, key: char) {
        unimplemented!();
    }
}
