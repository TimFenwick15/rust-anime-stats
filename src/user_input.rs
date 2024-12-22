use std::io;

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
    AnimeOrManga(String), // anime, manga
    Rating(String),       // 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    Status(String),       // Completed, Dropped, On-Hold, Watching (anime only), Plan to Watch (anime only), Reading (Manga only), Plan to Read (Manga only)
    Name(String),         // a, b, c, ...
}

impl UserInputParsed {
    fn parse(user_input: String) -> UserInputParsed {
        let binding = user_input.to_lowercase();
        let lower_user_input = binding.trim();

        match lower_user_input {
            "quit" | "exit" => UserInputParsed::Quit,
            "reset" => UserInputParsed::Reset,

            "anime" => UserInputParsed::AnimeOrManga(String::from("anime")),
            "manga" => UserInputParsed::AnimeOrManga(String::from("manga")),

            "completed" | "dropped" | "on-hold" | "watching" | "plan to watch"
            | "reading" | "plan to read" => UserInputParsed::Status(String::from(lower_user_input)),
            
            "10" => UserInputParsed::Rating(String::from(lower_user_input)),

            // To avoid having to write out the alphabet
            test if test.len() == 1 => {
                let c: char = lower_user_input.chars().next().expect("No reason");
                match c {
                    '1'..'9' => UserInputParsed::Rating(String::from(c)),
                    'a'..'z' => UserInputParsed::Name(String::from(c)),
                    _ => UserInputParsed::None,
                }
            }

            _ => UserInputParsed::None,
        }
    }
}

pub struct UserInput {
    index: usize,
    filters: [UserInputParsed; 5],
}

impl UserInput {
    pub fn new() -> UserInput {
        UserInput {
            index: 0,
            //filters: [UserInputParsed::None.clone(); 5] // Ideally don't? These objects will be binned
            filters: [UserInputParsed::None, UserInputParsed::None, UserInputParsed::None, UserInputParsed::None, UserInputParsed::None]
        }
    }

    pub fn run(&mut self) -> Exit {
        let mut buffer = match read_input() {
            Ok(v) => v,
            Err(_e) => String::new(),
        };

        let input = UserInputParsed::parse(buffer);

        println!("new input: {:?}", input);

        match input {
            UserInputParsed::Quit => Exit::Quit,
            UserInputParsed::Reset => {
                for filter in self.filters.iter_mut() {
                    *filter = UserInputParsed::None;
                }
                Exit::Stay
            },
            UserInputParsed::AnimeOrManga(_) | UserInputParsed::Rating(_)
                    | UserInputParsed::Status(_) | UserInputParsed::Name(_) => {
                self.filters[self.index] = input;
                self.index += 1;
                if self.index >= 5 {
                    self.index = 0;
                }
                println!("Filters: {:?}, {:?}, {:?}, {:?}, {:?}",
                    self.filters[0], self.filters[1], self.filters[2], self.filters[3], self.filters[4]);
                Exit::Stay
            },
            _ => Exit::Stay,
        }
    }
}

fn read_input() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}
