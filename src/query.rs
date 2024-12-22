use crate::mal;

#[derive(Debug)]
pub enum Filter {
    None,
    AnimeOrManga(String), // anime, manga
    Rating(String),       // 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    Status(String),       // Completed, Dropped, On-Hold, Watching (anime only), Plan to Watch (anime only), Reading (Manga only), Plan to Read (Manga only)
    Name(String),         // a, b, c, ...
}

/*pub struct Query {
    filters: [Filter; 5]
}

impl Query {
    pub fn new() -> Query {
        Query {
            filters: [Filter::None, Filter::None, Filter::None, Filter::None, Filter::None],
        }
    }

    pub fn add(filter: Filter) {

    }
}*/

pub fn im_here() {
    println!("hello world");
}