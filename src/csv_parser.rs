use std::fs::File;
use std::io::{BufReader, Error};

use crate::config::Config;

pub fn read(cfg: &Config) -> Result<Vec<String>, Error> {
    let input = File::open(&cfg.filename).unwrap();
    let buffered = BufReader::new(input);

    let mut reader = csv::ReaderBuilder::new().from_reader(buffered);

    let mut records: Vec<String> = Vec::new();
    for record in reader.records() {
        let record = record.unwrap();
        let record = record.get(cfg.column as usize).unwrap();
        records.push(String::from(record));
    }

    Ok(records)
}
