pub mod config;
mod csv_parser;

pub fn run(cfg: &config::Config) {
    let results = csv_parser::read(cfg).unwrap();
    for (index, record) in results.iter().enumerate() {
        println!("{}{}  ansible_host={}  ansible_port=22", cfg.prefix, index, record);
    }
}
