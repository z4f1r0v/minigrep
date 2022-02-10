use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    if config.count_lines_only {
        let result = count_lines(config.query, &contents);
        println!("{}", result)
    } else if config.case_sensitive {
        let results = search(config.query, &contents);
        results.iter().for_each(|x| println!("{}", x));
    } else {
        let results = search_case_sensitive(config.query, &contents);
        results.iter().for_each(|x| println!("{}", x));
    };

    Ok(())
}

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
    pub count_lines_only: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let filename = &args[2];
        let case_sensitive = args.contains(&"--case-sensitive".to_string());
        let count_lines_only = args.contains(&"-c".to_string());

        Ok(Config { query, filename, case_sensitive, count_lines_only })
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

fn count_lines<'a>(query: &'a str, contents: &'a str) -> i32 {
    contents.lines().fold(0, |acc, l| {
        if l.contains(query) {
            acc + 1
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

    #[test]
    fn test_count_lines() {
        let query = "rust";
        let contents = "\
rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(2, count_lines(query, contents));
    }
}
