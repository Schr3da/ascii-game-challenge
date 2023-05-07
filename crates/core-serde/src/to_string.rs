use serde::Serialize;
use serde_json::*;

pub fn struct_to_string<T>(next: T) -> String
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
