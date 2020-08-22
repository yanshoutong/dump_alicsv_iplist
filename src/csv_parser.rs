use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::config::Config;

pub fn read(cfg: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let input = File::open(&cfg.filename)?;
    let buffered = BufReader::new(input);

    let mut reader = csv::ReaderBuilder::new().from_reader(buffered);

    let mut records: Vec<String> = Vec::new();
    for record in reader.records() {
        let record = record?;

        match record.get(cfg.column as usize) {
            Some(content) => records.push(String::from(content)),
            _ => eprintln!("cannot get record at column {}", cfg.column),
        }
    }

    Ok(records)
}
