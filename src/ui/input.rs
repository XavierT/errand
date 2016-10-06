
/// Mode of input similar to Vim input mode.
///
/// In each mode the key will have a different meaning
pub enum InputMode {
    /// Default game mode
    Default,
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
        InputHandler { mode: InputMode::Default }
    }

    pub fn change_mode(&mut self, new_mode: InputMode) {
        self.mode = new_mode;
    }

    pub fn process_key(&self, key: char) {

        match key {
            'h' => info!("west"),
            'j' => info!("south"),
            'k' => info!("north"),
            'l' => info!("left"),
            'y' => info!("left"),
            'u' => info!("left"),
            'b' => info!("left"),
            'n' => info!("left"),
            _ => info!("error"),
        }

    }
}
