use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub line_number: bool,
    pub count_only: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let mut ignore_case = false;
        let mut line_number = false;
        let mut count_only = false;

        for arg in args.iter().skip(3) {
            if (arg == "-ic" || arg == "--ignore-case") && !ignore_case {
                ignore_case = true;
            } else if (arg == "-ln" || arg == "--line-number") && (!line_number && !count_only) {
                line_number = true;
            } else if (arg == "-co" || arg == "--count-only") && (!count_only && !line_number) {
                count_only = true;
            } else {
                return Err("Invalid flags/combination provided");
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
            line_number,
            count_only,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    let lines = if config.ignore_case {
        search_case_insesitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    if config.count_only {
        println!("Count: {}", lines.len());
    } else {
        for (line_num, line) in &lines {
            if config.line_number {
                println!("{} (line={})", line, line_num);
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<(usize, &'a str)> {
    let mut res = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        if line.contains(&query) {
            res.push((idx + 1, line.trim()));
        }
    }
    res
}

pub fn search_case_insesitive<'a>(query: &str, content: &'a str) -> Vec<(usize, &'a str)> {
    let mut res = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            res.push((idx + 1, line.trim()));
        }
    }
    res
}
