use std::{fs, error::Error, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filename)?;

    
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }


    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results: Vec<&str> = Vec::new();
    contents.lines().for_each(|line: &str| {
        if line.contains(query) {
            results.push(line);
        }
    });

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    
    let query: String = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    
    contents.lines().for_each(|line: &str| {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    });

    results
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        return Ok(Config { query, filename, ignore_case});
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, productive, fast.
Pick Three.";
        assert_eq!(vec!["safe, productive, fast."], search(query, contents));
    }

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, productive, fast.
Pick Three.
Duct Tape";
        assert_eq!(vec!["safe, productive, fast."], search(query, contents));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, productive, fast.
Pick Three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}