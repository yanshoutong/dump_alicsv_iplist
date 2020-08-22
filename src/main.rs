extern crate clap;

use clap::{App, Arg};
use dumpip::config::Config;
use dumpip::run;

fn parse_args() -> Config {
    let matches = App::new("Dump IP List")
        .version("1.0.0")
        .author("harold <harold@gmail.com>")
        .about("Dump IP list from file.csv from Aliyun console in ansible format")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input csv file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("prefix")
                .help("Sets the prefix string to use")
                .short("p")
                .long("prefix")
                .value_name("PREFIX")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("column")
                .help("Sets the column corresponding to public ip in csv file")
                .short("c")
                .long("column")
                .value_name("COLUMN")
                .takes_value(true),
        )
        .get_matches();

    let prefix = matches.value_of("config").unwrap_or("ecs_");
    let column = matches
        .value_of("column")
        .unwrap_or("1")
        .parse::<u8>()
        .unwrap_or(0);

    let filename = matches.value_of("INPUT").unwrap();

    Config::new(filename, prefix, column)
}

fn main() {
    let result = parse_args();
    println!("{:#?}", result);

    run(&result);
}
