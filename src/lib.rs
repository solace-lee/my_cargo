use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("第一个参数{}", config.query);
    println!("第二个参数{}", config.filename);
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("with text: \n{}", line);
    }
    return Ok(());
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 3 {
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config { query, filename })
        } else {
            Err("not enough arguments")
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
}
