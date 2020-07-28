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

    let results =
        if (config.flag.max_count != 0) && ((config.flag.max_count as usize) < results.len()) {
            &results[..config.flag.max_count as usize]
        } else {
            &results[..]
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
    pub fn new(args: env::Args) -> Result<Config, String> {
        let args = args.collect();
        let (flag, others) = Flag::from_vec(&args)?;

        let mut others = others.iter();
        others.next();

        let query = if let Some(arg) = others.next() {
            arg
        } else {
            return Err("Not enough arguments".to_string());
        };

        let filename = if let Some(arg) = others.next() {
            arg
        } else {
            return Err("Not enough arguments".to_string());
        };

        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string(),
            flag,
        })
    }
}

pub struct Flag {
    pub ignore_case: bool,
    pub max_count: u8,
}

// it should have more options and logics
impl Flag {
    pub fn from_vec<'a>(flag_strs: &'a Vec<String>) -> Result<(Flag, Vec<&'a String>), String> {
        let mut iter = flag_strs.iter();
        let mut flags: Flag = Default::default();
        let mut arguments = Vec::new();

        while let Some(flag_str) = iter.next() {
            if flag_str.chars().nth(0).unwrap() != '-' {
                arguments.push(flag_str);
                continue;
            }

            if flag_str.chars().nth(1).unwrap() != '-' {
                let mut short_iter = flag_str.chars();
                short_iter.next();

                while let Some(flag_ch) = short_iter.next() {
                    match flag_ch {
                        'i' => flags.ignore_case = true,
                        'm' => {
                            let max_num: String = short_iter.collect();
                            flags.max_count = if max_num != "" {
                                if let Ok(num) = max_num.parse() {
                                    num
                                } else {
                                    return Err("invalid max count".to_string());
                                }
                            } else if let Some(s) = iter.next() {
                                if let Ok(num) = s.parse() {
                                    num
                                } else {
                                    return Err("invalide max count".to_string());
                                }
                            } else {
                                return Err("invalid max count".to_string());
                            };
                            break;
                        }
                        _ => return Err(format!("can't parse option {}", flag_ch)),
                    }
                }
            } else {
                let mut split_flag = flag_str.split('=');
                match split_flag.next().unwrap() {
                    "--ignore-case" => flags.ignore_case = true,
                    "--max-count" => {
                        flags.max_count = if let Some(s) = split_flag.next() {
                            if let Ok(num) = s.parse() {
                                num
                            } else {
                                return Err("invalid max count".to_string());
                            }
                        } else if let Some(s) = iter.next() {
                            if let Ok(num) = s.parse() {
                                num
                            } else {
                                return Err("invalid max count".to_string());
                            }
                        } else {
                            return Err("invalid max count".to_string());
                        }
                    }
                    _ => return Err(format!("can't parse option {}", flag_str)),
                }
            }
        }

        Ok((flags, arguments))
    }
}

impl Default for Flag {
    fn default() -> Self {
        Self {
            ignore_case: false,
            max_count: 0,
        }
    }
}
