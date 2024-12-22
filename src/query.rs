use crate::mal;

#[derive(Debug)]
pub enum Filter {
    None,
    AnimeOrManga(String), // anime, manga
    Rating(String),       // 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    Status(String),       // Completed, Dropped, On-Hold, Watching (anime only), Plan to Watch (anime only), Reading (Manga only), Plan to Read (Manga only)
    Name(String),         // a, b, c, ...
}

pub struct Query {
    index: usize,
    filters: [Filter; 5],
}

impl Query {
    pub fn new() -> Query {
        Query {
            index: 0,
            filters: [Filter::None, Filter::None, Filter::None, Filter::None, Filter::None],
        }
    }

    pub fn print(&mut self) {
        println!("Filters: {:?}, {:?}, {:?}, {:?}, {:?}",
        self.filters[0], self.filters[1], self.filters[2], self.filters[3], self.filters[4]);
    }

    pub fn reset(&mut self) {
        for filter in self.filters.iter_mut() {
            *filter = Filter::None;
        }
    }

    pub fn add(&mut self, filter: Filter) {
        self.filters[self.index] = filter;
        self.index += 1;
        if self.index >= 5 {
            self.index = 0;
        }
        self.print();
    }

    pub fn search(&mut self) {
        self.print();
    }
}
