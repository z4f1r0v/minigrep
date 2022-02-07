use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(config.query, &contents)
    } else {
        search_case_sensitive(config.query, &contents)
    };

    results.iter()
        .for_each(|x| println!("{}", x));
    Ok(())
}

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let filename = &args[2];
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().fold(Vec::new(), |mut acc, l| {
        if l.contains(query) {
            acc.push(l);
            acc
        } else {
            acc
        }
    })
}

fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let lowercased_query = query.to_lowercase();
    contents.lines().fold(Vec::new(), |mut acc, l| {
        if l.to_lowercase().contains(&lowercased_query) {
            acc.push(l);
            acc
        } else {
            acc
        }
    })
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_sensitive(query, contents));
    }
}
