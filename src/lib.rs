use std::error::Error;
use std::fs;

use crate::config::{count_lines, search_case_insensitive, search_case_sensitive};
pub use crate::config::Config;

mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    if config.count_lines_only {
        let result = count_lines(config.query, &contents);
        println!("{}", result)
    } else if config.ignore_case {
        let results = search_case_insensitive(config.query, &contents);
        results.iter().for_each(|x| println!("{}", x));
    } else {
        let results = search_case_sensitive(config.query, &contents);
        results.iter().for_each(|x| println!("{}", x));
    };

    Ok(())
}