use std::fs::File;
use std::io::{BufReader, Error};

use crate::config::Config;

pub fn read(cfg: &Config) ->Result<Vec<String>, Error> {
    let input = File::open(&cfg.filename)?;
    let buffered = BufReader::new(input);

    let mut reader = csv::ReaderBuilder::new()
        .from_reader(buffered);

    let mut records: Vec<String> = Vec::new();
    for record in reader.records() {
        let record = record?;
        println!("{}", &record[0]);
    }

    Ok(records)

}