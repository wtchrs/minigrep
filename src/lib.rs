use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.is_case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
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
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: env::Args) -> Result<Config, &'static str> {
        let (flags, others): (Vec<String>, Vec<String>) =
            args.partition(|s| s.chars().nth(0).unwrap() == '-');

        let mut others = others.iter();
        others.next();

        let query = match others.next() {
            Some(arg) => arg,
            None => return Err("Not enough args"),
        };

        let filename = match others.next() {
            Some(arg) => arg,
            None => return Err("Not enough args"),
        };

        let mut flag_str = String::new();
        for s in flags {
            flag_str.push_str(&s);
        }

        let case_sensitive = match flag_str.chars().filter(|ch| *ch == 'i').next() {
            Some(_) => false,
            None => true,
        };

        let query = query.to_string();
        let filename = filename.to_string();

        Ok(Config {
            query,
            filename,
            is_case_sensitive: case_sensitive,
        })
    }
}
