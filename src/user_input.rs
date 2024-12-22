use std::io;
use crate::query;

#[derive(PartialEq)] // This shouldn't be part of a user input module
pub enum Exit {
    Stay,
    Quit,
}

#[derive(Debug)]
enum UserInputParsed {
    None,
    Reset,                // Clear filters
    Quit,
    Search,
    Filter(query::Filter),
}

impl UserInputParsed {
    fn parse(user_input: String) -> UserInputParsed {
        let binding = user_input.to_lowercase();
        let lower_user_input = binding.trim();

        match lower_user_input {
            "quit" | "exit" => UserInputParsed::Quit,
            "reset" => UserInputParsed::Reset,
            "search" | "find" | "get" => UserInputParsed::Search,

            "anime" => UserInputParsed::Filter(query::Filter::AnimeOrManga(String::from("anime"))),
            "manga" => UserInputParsed::Filter(query::Filter::AnimeOrManga(String::from("manga"))),

            "completed" | "dropped" | "on-hold" | "watching" | "plan to watch"
            | "reading" | "plan to read" => UserInputParsed::Filter(query::Filter::Status(String::from(lower_user_input))),
            
            "10" => UserInputParsed::Filter(query::Filter::Rating(String::from(lower_user_input))),

            // To avoid having to write out the alphabet
            test if test.len() == 1 => {
                let c: char = lower_user_input.chars().next().expect("No reason");
                match c {
                    '1'..'9' => UserInputParsed::Filter(query::Filter::Rating(String::from(c))),
                    'a'..'z' => UserInputParsed::Filter(query::Filter::Name(String::from(c))),
                    _ => UserInputParsed::None,
                }
            },
            _ => UserInputParsed::None,
        }
    }
}

pub struct UserInput {
    index: usize,
    filters: [query::Filter; 5],
}

impl UserInput {
    pub fn new() -> UserInput {
        UserInput {
            index: 0,
            filters: [query::Filter::None, query::Filter::None, query::Filter::None, query::Filter::None, query::Filter::None]
        }
    }

    fn print(&mut self) {
        println!("Filters: {:?}, {:?}, {:?}, {:?}, {:?}",
            self.filters[0], self.filters[1], self.filters[2], self.filters[3], self.filters[4]);
        query::im_here();
    }

    fn read_input(&mut self) -> Result<String, io::Error> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        Ok(buffer)
    }

    fn reset(&mut self) {
        for filter in self.filters.iter_mut() {
            *filter = query::Filter::None;
        }
    }

    fn add_filter(&mut self, filter: query::Filter) {
        self.filters[self.index] = filter;
        self.index += 1;
        if self.index >= 5 {
            self.index = 0;
        }
        self.print();
    }

    fn query(&mut self) {
        self.print();
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
                self.reset();
                Exit::Stay
            },
            UserInputParsed::Search => {
                self.query();
                Exit::Stay
            },
            UserInputParsed::Filter(f) => {
                self.add_filter(f);
                Exit::Stay
            },
            _ => Exit::Stay,
        }
    }
}
