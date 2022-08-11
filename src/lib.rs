use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents){
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
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config { query, filename});
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
}