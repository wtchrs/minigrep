use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.flag.case_sensitive {
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
    pub flag: Flag,
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

        let flag = match Flag::from_vec(flags) {
            Ok(f) => f,
            Err(s) => return Err(s),
        };

        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string(),
            flag,
        })
    }
}

pub struct Flag {
    case_sensitive: bool,
}

impl Flag {
    fn from_vec(flag_vec: Vec<String>) -> Result<Flag, &'static str> {
        let (short_flag, long_flag): (Vec<&String>, Vec<&String>) =

        //TODO: add more options and two-hyphen options

        flag_vec.iter().partition(|s| match s.chars().nth(1) {
            Some(ch) => {
                if ch != '-' {
                    true
                } else {
                    false
                }
            }
            None => true,
        });

        let mut flag_str = String::new();
        for s in short_flag {
            flag_str.push_str(&s);
        }

        let case_sensitive = match flag_str.chars().filter(|ch| *ch == 'i').next() {
            Some(_) => false,
            None => true,
        };

        Ok(Flag { case_sensitive })
    }
}
