use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl<'a> Config {
    const ENV_IGNORE_CASE: &'a str = "IGNORE_CASE";

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // First arg ignored (bin name)
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a file_path string"),
        };

        let ignore_case = env::var(Self::ENV_IGNORE_CASE).is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    #[test]
    fn one_result() {
        let query = "duct";

        assert_eq!(vec!["safe, fast, productive."], search(&query, &CONTENTS))
    }

    #[test]
    fn case_insensitive_on() {
        let query = "rUsT";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(&query, &CONTENTS)
        )
    }
}
