
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

struct InputHandler{
    mode : InputMode,
}

impl InputHandler{
    pub fn new() -> InputHandler{
        InputHandler{
            mode : InputMode::Default,
        }
    }

    pub fn process_key(key: i32){

    }

}


