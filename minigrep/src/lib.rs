/* $ cargo run -- frog poem.txt */
/* cargo run -- body poem.txt */

use std::error::Error;
use std::fs;

/* Group configuration values using a struct */
pub struct Config {
    pub query: String,
    pub file_path: String,
}

/* Create a factory constructor for struct Config */
impl Config {
    // previous constructor declaration
    // fn new(args: &[String]) -> Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // determine the number of arguments passed from CLI
        // one of valid approaches is to panic when argument count is less than 3
        // if args.len() < 3 {
        //     panic!("not enough arguments");
        // }
        // the better approach would be to return a Result<T, E> struct
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Config { query, file_path })
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
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
