use std::fs::File;
use std::io::prelude::*;

fn read_anime_list(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main () {
    let data = match read_anime_list("animelist.xml") {
        Ok(value) => value,
        Err(_) => String::from("")
    };

    println!("{}", data);
}
