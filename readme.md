# Rust
## Cargo
- `cargo new anime-stats`
- `cd anime-stats`
- `cargo run`
- `rustc --version`

### xml
- `use xmlparser;` raises `no external crate 'xmlparser'`
- `cargo install xmlparser` doesn't work, xmlparser is a cargo library, not a binary
- `cargo add xmlparser` does work
- Now the code does work using `use xmlparser;`
- This library wasn't working as I expected and struggled to find examples. Decided to use another library
- `cargo remove xmlparser`
- `cargo add xml`

## Docs
File handling: https://doc.rust-lang.org/std/fs/struct.File.html

XML parsing: https://docs.rs/xmlparser/latest/xmlparser/

## match
Surprised this works, each match arm has a different type.

```
    let data = match read_anime_list("animelist.xml") {
        Ok(value) => value,
        Err(_) => {
            println!("File not found");
            std::process::exit(1);
        }
    };
```

From testing, `break` and `continue` work like this too. The semi-colon is optional in this case.

This does not work:

```
Err(_) => {
    if true {
        std::process::exit(1);
    }
}
```

## ?
`?` syntax is only allowed in a function that returns `Result<T, io::Error>`.

This line:

```
let str = fs::read_to_string(filepath)?;
```

Is equivalent to this:

```
let str = match fs::read_to_string(filepath) {
    Ok(contents) => contents,
    Err(error) => return Err(error)
};
```

## Crates/Modules
I don't understand this well yet and probably haven't correctly organised my source files. This post helped getting started: https://stackoverflow.com/questions/69868409/how-do-i-split-my-rust-program-into-many-files

## Other Rust Notes
Rust embedded book: https://doc.rust-lang.org/stable/embedded-book/

# MyAnimeList
Can export anime or manga ratings for your account here: https://myanimelist.net/panel.php?go=export

`gzip -d animelist.xml.gz`

# Code Design
## Data
Anime and manga xml have similar fields, but not the same tag names. We can read both in the same way but will need to handle the different tag names.

Two entries can have the same title but need to be differentiated. The ID should be unique, or we could add our own ID.

- User profile data
- Anime/manga
  - Name (Romanji only)
  - Rating
  - Status
    - Completed
    - Dropped
    - On-Hold
    - Watching - anime only
    - Plan to Watch - anime only
    - Reading - Manga only
    - Plan to Read - Manga only

## Features
- List all
- List all with rating
- List by name starting from letter
- List all with status
- Or a combination of above filters
- Console app to accept rating or score and return result
- Web UI to show result
- Anime/manga switch

## Code Layout
```
Console Input Parser > Console Interface > Data Interface > MAL > XML file
Web UI               > Web Server        >
```

### Console Input Parser
- Writes instructions to terminal
- Accept command - type (anime or manga), rating, status, first letter. Then accepts argument for the command
- Sends requests to the console interface
- Writes the response to the terminal

### Console Interface
- Receives commands from Console Input Parser
- Sends data requests to the Data Interface
- Passes the response back to Console Input Parser
- Can also command the Web Server if we like (probably just start stop)
- I like including this because it keeps console commands and data queries seperate

### Data Interface
- Receives requests for data
- Responds with a payload
- Has data sources - just MAL in my case

### MAL
- Reads the MAL XML files
- Supply dicts to the Data Interface in a format I define
- This will be the only part of the code that knows about MAL's data format
