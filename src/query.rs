use crate::mal::Data;
use crate::anime_manga::AnimeManga;
use std::mem::discriminant;

#[derive(Debug)]
pub enum Filter {
    None,
    AnimeOrManga(String), // anime, manga
    Rating(String),       // 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    Status(String),       // Completed, Dropped, On-Hold, Watching (anime only), Plan to Watch (anime only), Reading (Manga only), Plan to Read (Manga only)
    Name(String),         // a, b, c, ...
}

// We need to be able to overwrite filters with the same type but differing data.
// Credit to https://users.rust-lang.org/t/comparing-tuple-like-enum/88332/4
impl PartialEq<Self> for Filter {
    fn eq(&self, rhs: &Self) -> bool {
        discriminant(self) == discriminant(rhs)
    }
}

pub struct Query {
    filters: [Filter; 5], // Size is hard coded, there doesn't seem to be a stable way to get this number yet
}

impl Query {
    pub fn new() -> Query {
        Query {
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

    pub fn add(&mut self, new_filter: Filter) {
        // New filters with the same type as an existing one will overwrite.
        // This is a lazy way to handle two different filters with the same type
        // since my filters always AND with eachother.
        // Eg filtering scores 7 and 8 would fail because the score can't equal both 7 and 8.
        let mut none_index = 0;
        for (i, filter) in self.filters.iter_mut().enumerate() {
            if *filter == new_filter {
                *filter = new_filter;
                self.print();
                return;
            }
            else if *filter == Filter::None {
                none_index = i;
            }
        }
        self.filters[none_index] = new_filter;
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
