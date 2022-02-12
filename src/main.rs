extern crate core;

use std::process;

use clap::{App, Arg, arg, ArgMatches};

use minigrep::Config;

fn main() {
    let matches: ArgMatches = App::new("minigrep")
        .about("Minigrep")
        .arg(
            Arg::new("case-sensitive")
                .long("case-sensitive")
                .help("Enable case sensitivity")
                .takes_value(false)
        )
        .arg(
            Arg::new("count")
                .short('c')
                .help("Return the amount matches")
                .takes_value(false)
        )
        .arg(
            Arg::new("number-lines")
                .short('n')
                .help("Number each line matching query staring from 1")
                .takes_value(false)
        )
        .arg(arg!([QUERY]))
        .arg(arg!([FILENAME]))
        .get_matches();

    let config = Config::new(&matches)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1)
        });

    let query = config.query;
    let filename = config.filename;

    println!("Searching for {}", query);
    println!("In file {}", filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
