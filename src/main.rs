extern crate core;

use std::process;

use clap::{App, Arg, ArgMatches};

use picogrep::Config;

fn main() {
    let matches: ArgMatches = App::new("picogrep")
        .about("A miniature version of grep")
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Return the amount matches")
                .takes_value(false)
        )
        .arg(
            Arg::new("case_insensitive")
                .short('i')
                .long("case-insensitive")
                .help("Perform a case insensitive search")
                .takes_value(false)
        )
        .arg(
            Arg::new("QUERY")
                .help("The string to search for")
                .required(true)
        )
        .arg(
            Arg::new("FILENAME")
                .help("The file to search")
                .required(true)
        )
        .get_matches();

    let config = Config::parse(&matches)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1)
        });

    if let Err(e) = Config::new(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
