#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub prefix: String,
    pub column: u8,
}

impl Config {
    pub fn new(filename: &str, prefix: &str, column: u8) -> Self {
        Config {
            filename: String::from(filename),
            prefix: String::from(prefix),
            column: column,
        }
    }
}
