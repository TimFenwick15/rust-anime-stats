use std::fs;
use std::fs::File;
use std::io::prelude::*;

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
    /* This is a simpler way to do this, but want an example using File above
    let data = match fs::read_to_string("animelist.xml") {
        Ok(value) => value,
        Err(error) => {
            println!("File not found: {}", error);
            std::process::exit(1);
        }
    };*/
    println!("{}", data);

    // Parse XML
    //
}
