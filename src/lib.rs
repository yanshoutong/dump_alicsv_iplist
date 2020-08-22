pub mod config;
mod csv_parser;

pub fn run(cfg: &config::Config) {
    let results = csv_parser::read(cfg).unwrap();
    for (index, record) in results.iter().enumerate() {
        println!("{}\t\t{}\t ansible_port=22",
            format!("{}{:03}", cfg.prefix, index),
            format!("ansible_host={:20}", record)
        );

    }
}
