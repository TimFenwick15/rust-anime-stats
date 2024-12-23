use crate::mal::Data;
use crate::anime_manga::AnimeManga;

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

    pub fn search(&mut self) -> Vec<AnimeManga>{
        let mut filtered_data = Vec::new();

        let mut data = Data::new();
        data.search();

        for item in data.items {
            let mut include = true;

            for filter in &self.filters {
                let exclude = match filter {
                    Filter::None => false,
                    Filter::AnimeOrManga(x) => item.format != *x,
                    Filter::Rating(x) => item.rating != *x,
                    Filter::Status(x) => item.status.to_lowercase() != *x,
                    Filter::Name(x) => item.name.to_lowercase().as_bytes()[0] != x.as_bytes()[0],
                };
                if exclude {
                    include = false;
                    break;
                }
            }

            if include {
                filtered_data.push(item.clone());
            }
        }

        return filtered_data;
    }
}
