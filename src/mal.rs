use std::fs::File;
use std::io::prelude::*;

fn read_anime_list(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn query() {
    let data = match read_anime_list("data/animelist_1734298612_-_16433979.xml") {
        Ok(value) => value,
        Err(error) => {
            println!("File not found: {}", error);
            std::process::exit(1);
        }
    };
    println!("{}", &data[..100]);
}
