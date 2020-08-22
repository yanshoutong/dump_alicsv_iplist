pub mod config;
mod csv_parser;

pub fn run(cfg: &config::Config) {
    let results = csv_parser::read(cfg).unwrap();
    for record in results {
        println!(">>> {}", record);
    }
}
