mod user_input; // Not sure I'm doing this right, if main declares the modules, other files can "use" them
mod query;
mod mal;

use crate::user_input::{Exit, UserInput};

fn main () {
    let mut ui = UserInput::new();
    loop {
        if Exit::Quit == ui.run() {
            break;
        }
    }
}
