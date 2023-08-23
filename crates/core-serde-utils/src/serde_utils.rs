use serde_json::*;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub struct SerdeUtils;

impl SerdeUtils {
    pub fn parse_string<T>(next: &String) -> T
    where
        T: Default + Clone + for<'a> Deserialize<'a>,
    {
        from_str(next.as_str()).unwrap_or_default()
    }

    pub fn to_string<T>(next: T) -> String
    where
        T: Default + Clone + Serialize,
    {
        match to_string_pretty(&next) {
            Ok(n) => n,
            Err(_) => {
                let default = T::default();
                serde_json::to_string_pretty(&default).unwrap_or_default()
            }
        }
    }

    pub fn load_json_file<T>(next: &str) -> T
    where
        T: Default + Clone + Serialize + for<'a> Deserialize<'a>,
    {
        let path = Path::new(next);

        let result = fs::read_to_string(path);
        match result {
            Ok(c) => Self::parse_string::<T>(&c),
            Err(e) => {
                println!("{e}");
                T::default()
            }
        }
    }
}
