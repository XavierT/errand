
use std::process;

/// Mode of input similar to Vim input mode.
///
/// In each mode the key will have a different meaning
enum InputMode{
    /// Default game mode
    Default,
    /// Move cursor freely around to identify things
    Explore,
    /// Menu
    Menu,
    /// Yes or no question
    Yes_or_No,
    /// Help files
    Help,
}

pub struct InputHandler{
    mode : InputMode,
}

impl InputHandler{
    pub fn new() -> InputHandler{
        InputHandler{
            mode : InputMode::Default,
        }
    }

    pub fn process_key(&self, key: char){

        match key {
            'h' => println!("west"),
            'j' => println!("south"),
            'k' => println!("north"),
            'l' => println!("left"),
            'y' => println!("left"),
            'u' => println!("left"),
            'b' => println!("left"),
            'n' => println!("left"),
             _  => println!("error"),
        }

    }

}


