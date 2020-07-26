use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.flag.ignore_case {
        search_ignore_case(&config.query, &contents)
    } else {
        search(&config.query, &contents)
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

pub fn search_ignore_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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

        let flag = Flag::from_vec(flags)?;

        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string(),
            flag,
        })
    }
}

pub struct Flag {
    pub ignore_case: bool,
}

impl Flag {
    pub fn from_vec(flag_vec: Vec<String>) -> Result<Flag, &'static str> {
        let (short_flag, long_flag): (Vec<&String>, Vec<&String>) =
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

        let mut short_flag_str = String::new();
        for s in short_flag {
            short_flag_str.push_str(&s);
        }
        let short_flag_str: String = short_flag_str.chars().filter(|ch| *ch != '-').collect();

        let (ignore_case, short_flag_str) = Flag::short_flag_parse(short_flag_str, 'i');
        let (ignore_case_long, long_flag) = Flag::long_flag_parse(long_flag, "--ignore-case");
        let ignore_case = if ignore_case_long { true } else { ignore_case };

        let (no_ignore_case, long_flag) = Flag::long_flag_parse(long_flag, "--no-ignore-case");
        let ignore_case = if no_ignore_case {
            if ignore_case {
                return Err("cannot use both \"--ignore-case(-i)\" and \"--no-ignore-case\" flags");
            } else {
                false
            }
        } else {
            ignore_case
        };

        //TODO: add more short options and two-hyphen options

        if (&short_flag_str[..] != "") | (long_flag.len() != 0) {
            Err("Unknown flag(s) in arguments")
        } else {
            Ok(Flag { ignore_case })
        }
    }

    fn short_flag_parse(flag_str: String, flag_ch: char) -> (bool, String) {
        let (match_flag, flag_str): (String, String) =
            flag_str.chars().partition(|ch| *ch == flag_ch);

        let match_flag = match &match_flag[..] {
            "" => false,
            _ => true,
        };

        (match_flag, flag_str)
    }

    fn long_flag_parse<'a>(flag_vec: Vec<&'a String>, flag_str: &str) -> (bool, Vec<&'a String>) {
        let (match_flag_vec, flag_vec): (Vec<&String>, Vec<&String>) =
            flag_vec.iter().partition(|s| &s[..] == flag_str);

        let match_flag = match match_flag_vec.iter().next() {
            Some(_) => true,
            None => false,
        };

        (match_flag, flag_vec)
    }
}
