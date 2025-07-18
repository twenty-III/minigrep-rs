use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub line_number: bool,
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

        for arg in args.iter().skip(3) {
            if (arg == "-ic" || arg == "--ignore-case") && !ignore_case {
                ignore_case = true;
            } else if arg == "-ln" || arg == "--line-number" && !line_number {
                line_number = true;
            } else {
                return Err("Invalid flags provided");
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
            line_number,
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
        if config.line_number {
            println!("{} (line={})", line, line_num);
        } else {
            println!("{}", line);
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
        let query = "hold";
        let content = "\
Hold fast to dreams
For if dreams die
Life is a broken-winged bird
That cannot fly.

Hold fast to dreams
For when dreams go
Life is a barren field
Frozen with snow.";

        assert_eq!(
            vec![(1, "Hold fast to dreams"), (6, "Hold fast to dreams")],
            search_case_insesitive(query, content)
        );
    }
}
