# Rust
## Cargo
- `cargo new anime-stats`
- `cd anime-stats`
- `cargo run`
- `rustc --version`

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


# MyAnimeList
Can export anime or manga ratings for your account here: https://myanimelist.net/panel.php?go=export

`gzip -d animelist.xml.gz`

# Other Rust Notes
Rust embedded book: https://doc.rust-lang.org/stable/embedded-book/

# Code Design
## Data
Anime and manga xml have similar fields, but not the same tag names. We can read both in the same way but will need to handle the different tag names.

Two entries can have the title but need to be differentiated. The ID should be unique, or we could add our own ID.

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
