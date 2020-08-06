use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let contents = contents.lines().enumerate().collect();

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

    let format_fn: Box<dyn Fn(&&(usize, &str)) -> _> = if config.flag.line_number {
        Box::new(|(index, s)| format!("{}:{}", index, s))
    } else {
        Box::new(|(_, s)| String::from(*s))
    };
    let results = results.iter().map(format_fn).collect::<Vec<_>>();

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a Vec<(usize, &str)>) -> Vec<&'a (usize, &'a str)> {
    contents
        .iter()
        .filter(|(_, line)| line.contains(query))
        .collect()
}

pub fn search_ignore_case<'a>(
    query: &str,
    contents: &'a Vec<(usize, &str)>,
) -> Vec<&'a (usize, &'a str)> {
    let query = query.to_lowercase();

    contents
        .iter()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
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
    pub line_number: bool,
    pub max_count: u8,
}

impl Default for Flag {
    fn default() -> Self {
        Self {
            ignore_case: false,
            line_number: false,
            max_count: 0,
        }
    }
}

// it should have more options and logics.
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
                flags.short_parse(&flag_str, &mut iter)?;
            } else {
                flags.long_parse(&flag_str, &mut iter)?;
            }
        }

        Ok((flags, arguments))
    }

    fn short_parse<T>(&mut self, short_str: &String, iter: &mut T) -> Result<(), String>
    where
        T: Iterator,
        T::Item: ToString,
    {
        let mut short_iter = short_str.chars();
        short_iter.next();

        while let Some(flag_ch) = short_iter.next() {
            match flag_ch {
                'i' => self.ignore_case = true,
                'n' => self.line_number = true,
                'm' => {
                    let max_num: String = short_iter.collect();
                    self.max_count = if max_num != "" {
                        if let Ok(num) = max_num.parse() {
                            num
                        } else {
                            return Err("invalid max count".to_string());
                        }
                    } else if let Some(s) = iter.next() {
                        if let Ok(num) = s.to_string().parse() {
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
        Ok(())
    }

    fn long_parse<T>(&mut self, long_str: &String, iter: &mut T) -> Result<(), String>
    where
        T: Iterator,
        T::Item: ToString,
    {
        let mut split_flag = long_str.split('=');
        match split_flag.next().unwrap() {
            "--ignore-case" => self.ignore_case = true,
            "--no-ignore-case" => self.ignore_case = false,
            "--line-number" => self.line_number = true,
            "--max-count" => {
                self.max_count = if let Some(s) = split_flag.next() {
                    if let Ok(num) = s.parse() {
                        num
                    } else {
                        return Err("invalid max count".to_string());
                    }
                } else if let Some(s) = iter.next() {
                    if let Ok(num) = s.to_string().parse() {
                        num
                    } else {
                        return Err("invalid max count".to_string());
                    }
                } else {
                    return Err("invalid max count".to_string());
                }
            }
            _ => return Err(format!("can't parse option {}", long_str)),
        }

        if let Some(_) = split_flag.next() {
            Err("invalid option arguments".to_string())
        } else {
            Ok(())
        }
    }
}
