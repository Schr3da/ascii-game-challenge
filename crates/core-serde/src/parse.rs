use serde::Deserialize;
use serde_json::*;

pub fn parse_string<T>(next: &String) -> T
where
    T: Default + Clone + for<'a> Deserialize<'a>,
{
    from_str(next.as_str()).unwrap_or_default()
}
