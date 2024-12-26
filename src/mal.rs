use crate::anime_manga::AnimeManga;

use std::fs::{File, read_dir};
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

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
        let _ = self.xml_read();
    }

    pub fn xml_read(&mut self) -> std::io::Result<()> {
        let files = read_dir("./data")?;
        for file in files {
            let file = file?;
            let file_path = file.path();

            let file_open = File::open(file_path)?;
            let file_open = BufReader::new(file_open); // Buffering is important for performance
            let parser = EventReader::new(file_open);

            let mut format = String::new();
            let mut title = String::new();
            let mut score = String::new();
            let mut status = String::new();
            let mut next_field = String::new();

            // Possible matches are listed here: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            for element in parser {
                match element {
                    Ok(XmlEvent::StartElement { name, .. }) => {
                        if name.local_name == "anime" {
                            format = String::from("anime");
                        }
                        else if name.local_name == "manga" {
                            format = String::from("manga");
                        }
                        else if name.local_name == "series_title" || name.local_name == "manga_title" {
                            next_field = String::from("series_title");
                        }
                        else if name.local_name == "my_score" {
                            next_field = String::from("my_score");
                        }
                        else if name.local_name == "my_status" {
                            next_field = String::from("my_status");
                        }
                    },
                    Ok(XmlEvent::CData(s)) | Ok(XmlEvent::Characters(s)) => {
                        if next_field == "series_title" {
                            title = s;
                            next_field = String::new();
                        }
                        else if next_field == "my_score" {
                            score = s;
                            next_field = String::new();
                        }
                        else if next_field == "my_status" {
                            status = s;
                            next_field = String::new();
                        }
                    },
                    Err(e) => println!("Error: {:?}", e),
                    _ => {},
                }

                if format != "" && title != "" && score != "" && status != "" {
                    self.items.push(AnimeManga {
                        format: format.clone(),
                        rating: score.clone(),
                        status: status.clone(),
                        name:   title.clone(),
                    });
                    title = String::new();
                    score = String::new();
                    status = String::new();
                }
            }
        }

        Ok(())
    }
}
