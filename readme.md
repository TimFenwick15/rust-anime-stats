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

# Other Notes
Rust embedded book: https://doc.rust-lang.org/stable/embedded-book/
