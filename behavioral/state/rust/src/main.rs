extern crate state;

use state::*;

fn main() {
    let mut context = Context::new();

    context.set_state(Box::new(StartState));

    context.set_state(Box::new(StopState));
}
