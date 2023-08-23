pub struct DigitFormatter;

impl DigitFormatter {
    pub fn prettify_i32(value: i32) -> String {
        match value {
            0..=9 => format!("0{}", value),
            _ => value.to_string(),
        }
    }
}
