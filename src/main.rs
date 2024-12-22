use std::fs::File;
use std::io::prelude::*;

mod user_input;

fn read_anime_list(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main () {
    // Read in data
    let data = match read_anime_list("animelist.xml") {
        Ok(value) => value,
        Err(error) => {
            println!("File not found: {}", error);
            std::process::exit(1);
        }
    };
    println!("{}", &data[..100]);

    let mut ui = user_input::UserInput::new();
    loop {
        if user_input::Exit::Quit == ui.run() {
            break;
        }
    }

    // Parse XML
    //
}
