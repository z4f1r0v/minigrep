use std::{error::Error, fs};

use clap::ArgMatches;
use regex::Regex;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub count_lines_only: bool,
    pub case_insensitive: bool,
}

impl<'a> Config<'a> {
    pub fn parse(matches: &'a ArgMatches) -> Result<Config<'a>, &'a str> {
        let count_lines_only = matches.get_flag("count");
        let case_insensitive = matches.get_flag("case_insensitive");
        let query = matches.get_one::<String>("QUERY").expect("Missing query.");
        let filename = matches.get_one::<String>("FILENAME").expect("Missing filename.");

        Ok(Config {
            query,
            filename,
            count_lines_only,
            case_insensitive,
        })
    }

    pub fn new(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;
        let results = filter_lines(contents, config.case_insensitive, config.query);

        if config.count_lines_only {
            println!("{}", results.len());
        } else {
            results.iter().for_each(|x| println!("{}", x));
        };

        Ok(())
    }
}

pub fn filter_lines(contents: String, case_insensitive: bool, query: &str) -> Vec<String> {
    let regex_pattern = if case_insensitive {
        format!(r"(?i){}", regex::escape(query))
    } else {
        format!(r"{}", regex::escape(query))
    };
    let regex = Regex::new(&regex_pattern).expect("Invalid regex");
    let red_highlight = "\x1b[31m$0\x1b[0m"; // ANSI escape code for red color

    contents
        .lines()
        .filter(|line| regex.is_match(line))
        .map(|line| regex.replace_all(line, red_highlight).to_string())
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
            filter_lines(contents.to_string(), false, query)
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
            vec!["\u{1b}[31mRust\u{1b}[0m:", "T\u{1b}[31mrust\u{1b}[0m me."],
            filter_lines(contents.to_string(), true, query)
        );
    }
}
