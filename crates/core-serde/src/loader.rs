use parse::parse_string;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::prelude::*; 

pub fn load_json_from_file<T>(next: &str) -> T
where
    T: Default + Clone + Serialize + for<'a> Deserialize<'a>,
{
    let path = Path::new(next);

    let result = fs::read_to_string(path);
    match result {
        Ok(c) => parse_string::<T>(&c),
        Err(_) => T::default(),
    }
}
