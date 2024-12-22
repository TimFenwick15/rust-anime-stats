use std::fs::File;
use std::io::prelude::*;
use crate::anime_manga::AnimeManga;

pub struct Data {
    pub items: Vec<AnimeManga>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            items: Vec::new(),
        }
    }

    pub fn search(&mut self) {
        let data = match self.read_anime_list("data/animelist_1734298612_-_16433979.xml") {
            Ok(value) => value,
            Err(error) => {
                println!("File not found: {}", error);
                std::process::exit(1);
            }
        };
        //println!("{}", &data[..100]);

        // Serialise xml string into Vec<AnimeManga>, self.items
        // Some fake data for testing
        self.items.push(AnimeManga {
            format: String::from("anime"),
            rating: String::from("9"),
            status: String::from("completed"),
            name:   String::from("Hibike! Euphonium"),
        });
        self.items.push(AnimeManga {
            format: String::from("anime"),
            rating: String::from("8"),
            status: String::from("completed"),
            name:   String::from("K-On!"),
        });
    }

    fn read_anime_list(&mut self, file: &str) -> std::io::Result<String> {
        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
