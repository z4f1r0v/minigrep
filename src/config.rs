use clap::ArgMatches;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
    pub count_lines_only: bool,
    pub number_lines: bool,
}

impl<'a> Config<'a> {
    pub fn new(matches: &'a ArgMatches) -> Result<Config<'a>, &'a str> {
        let query = matches.value_of("QUERY").expect("Missing query.");
        let filename = matches.value_of("FILENAME").expect("Missing filename.");
        let case_sensitive = matches.is_present("case-sensitive");
        let count_lines_only = matches.is_present("count");
        let number_lines = matches.is_present("number-lines");

        Ok(Config { query, filename, case_sensitive, count_lines_only, number_lines })
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

pub fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
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

pub fn count_lines<'a>(query: &'a str, contents: &'a str) -> i32 {
    contents.lines().fold(0, |acc, l| {
        if l.contains(query) {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn number_lines<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().fold(Vec::new(), |mut acc, l| {
        if l.contains(query) {
            acc.push(&format!("{}:{}", &acc.len() + 1, l));
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

    #[test]
    fn test_number_lines() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["1:safe, fast, productive."], number_lines(query, contents));
    }
}
