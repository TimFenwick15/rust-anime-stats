mod user_input; // Not sure I'm doing this right, if main declares the modules, other files can "use" them
mod query;
mod mal;

fn main () {
    let mut ui = user_input::UserInput::new();
    loop {
        if user_input::Exit::Quit == ui.run() {
            break;
        }
    }
}
