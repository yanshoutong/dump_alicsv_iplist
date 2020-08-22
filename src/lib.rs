mod csv_parser;
pub mod config;

pub fn run(cfg: &config::Config) {
    let results = csv_parser::read(cfg).unwrap();
    for record in results {
        println!("{}", record);
    }
}