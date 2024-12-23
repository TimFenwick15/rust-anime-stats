use std::io;
use crate::query::{Query, Filter};

#[derive(PartialEq)] // This shouldn't be part of a user input module
pub enum Exit {
    Stay,
    Quit,
}

#[derive(Debug)]
enum UserInputParsed {
    None,
    Reset, // Clear filters
    Quit,
    Search,
    Filter(Filter),
}

impl UserInputParsed {
    fn parse(user_input: String) -> UserInputParsed {
        let binding = user_input.to_lowercase();
        let lower_user_input = binding.trim();

        match lower_user_input {
            "quit" | "exit" => UserInputParsed::Quit,
            "reset" => UserInputParsed::Reset,
            "search" | "find" | "get" => UserInputParsed::Search,

            "anime" => UserInputParsed::Filter(Filter::AnimeOrManga(String::from("anime"))),
            "manga" => UserInputParsed::Filter(Filter::AnimeOrManga(String::from("manga"))),

            "completed" | "dropped" | "on-hold" | "watching" | "plan to watch"
            | "reading" | "plan to read" => UserInputParsed::Filter(Filter::Status(String::from(lower_user_input))),
            
            "10" => UserInputParsed::Filter(Filter::Rating(String::from(lower_user_input))),

            // To avoid having to write out the alphabet
            test if test.len() == 1 => {
                let c: char = lower_user_input.chars().next().expect("No reason");
                match c {
                    '1'..='9' => UserInputParsed::Filter(Filter::Rating(String::from(c))),
                    'a'..='z' => UserInputParsed::Filter(Filter::Name(String::from(c))),
                    _ => UserInputParsed::None,
                }
            },
            _ => UserInputParsed::None,
        }
    }
}

pub struct UserInput {
    data: Query,
}

impl UserInput {
    pub fn new() -> UserInput {
        UserInput {
            data: Query::new(),
        }
    }

    fn read_input(&mut self) -> Result<String, io::Error> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        Ok(buffer)
    }

    pub fn run(&mut self) -> Exit {
        let mut buffer = match self.read_input() {
            Ok(v) => v,
            Err(_e) => String::new(),
        };

        let input = UserInputParsed::parse(buffer);

        println!("new input: {:?}", input);

        match input {
            UserInputParsed::Quit => Exit::Quit,
            UserInputParsed::Reset => {
                self.data.reset();
                Exit::Stay
            },
            UserInputParsed::Search => {
                for x in self.data.search() {
                    println!("{}, {}, {}, {}", x.name, x.format, x.status, x.rating);
                }
                println!("");

                Exit::Stay
            },
            UserInputParsed::Filter(f) => {
                self.data.add(f);
                Exit::Stay
            },
            _ => Exit::Stay,
        }
    }
}
