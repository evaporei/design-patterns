pub trait State {
    fn do_action(&self);
}

pub struct StartState;

impl State for StartState {
    fn do_action(&self) {
        println!("Player is in start state");
    }
}

pub struct StopState;

impl State for StopState {
    fn do_action(&self) {
        println!("Player is in stop state");
    }
}

pub struct Context {
    state: Option<Box<State>>,
}

impl Context {
    pub fn new() -> Self {
        Context { state: None }
    }
    pub fn set_state(&mut self, state: Box<State>) {
        self.state = Some(state);
        if let Some(ref state) = self.state {
            state.do_action();
        }
    }
}
