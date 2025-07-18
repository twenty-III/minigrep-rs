use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        if args.len() > 4 {
            return Err("More than needed argumemts");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = if args.len() == 4 {
            if args[3] == "--ignore-case" {
                true
            } else {
                return Err("Unknown flag");
            }
        } else {
            false
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
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

    for (line_num, line) in &lines {
        println!("{} (line={})", line, line_num);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
productive.
Pick three.";

        assert_eq!(
            vec![(2, "safe, fast, productive."), (3, "productive.")],
            search(query, content)
        );
    }
    #[test]
    fn two_result() {
        let query = "dUcT";
        let content = "\
Rust:
safe, fast, productive.
productive.
Pick three.";

        assert_eq!(
            vec![(2, "safe, fast, productive."), (3, "productive.")],
            search_case_insesitive(query, content)
        );
    }
}
