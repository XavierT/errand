

enum InputContext{
    Default,
    Menu,
    Help,
}

struct InputHandler{
    ctx : InputContext,
}

impl InputHandler{
    pub fn new() -> InputHandler{
        InputHandler{
            ctx : InputContext::Default,
        }
    }

    pub fn process_key(key: i32){

    }

}


