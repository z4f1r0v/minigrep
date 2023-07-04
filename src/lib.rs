use std::{error::Error, fs};

use clap::ArgMatches;
use colored::Colorize;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub count_lines_only: bool,
    pub case_insensitive: bool,
}

impl<'a> Config<'a> {
    pub fn parse(matches: &'a ArgMatches) -> Result<Config<'a>, &'a str> {
        let count_lines_only = matches.is_present("count");
        let case_insensitive = matches.is_present("case_insensitive");
        let query = matches.value_of("QUERY").expect("Missing query.");
        let filename = matches.value_of("FILENAME").expect("Missing filename.");

        Ok(Config {
            query,
            filename,
            count_lines_only,
            case_insensitive,
        })
    }

    pub fn new(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;

        match (config.case_insensitive, config.count_lines_only) {
            (true, true) => {
                let result = search_case_insensitive(config.query, &contents).len();
                println!("{}", result);
            }
            (true, false) => {
                let results = search_case_insensitive(config.query, &contents);
                results.iter().for_each(|x| println!("{}", x));
            }
            (false, true) => {
                let result = search_case_sensitive(config.query, &contents).len();
                println!("{}", result);
            }
            (false, false) => {
                let results = search_case_sensitive(config.query, &contents);
                results.iter().for_each(|x| println!("{}", x));
            }
        };

        Ok(())
    }
}

pub fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<String> {
    contents
        .lines()
        .filter(|l| l.contains(query))
        .map(|l| l.replace(query, &format!("{}", query.red())))
        .collect()
}

pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<String> {
    let query_lower = query.to_lowercase();
    
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query_lower))
        .map(|line| {
            let highlighted_line = line
                .to_lowercase()
                .replace(&query_lower, &format!("{}", query.red()));
            highlighted_line
        })
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, pro\u{1b}[31mduct\u{1b}[0mive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["\u{1b}[31mrUsT\u{1b}[0m:", "t\u{1b}[31mrUsT\u{1b}[0m me."],
            search_case_insensitive(query, contents)
        );
    }
}
