use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("第一个参数{}", config.query);
    println!("第二个参数{}", config.filename);
    let contents = fs::read_to_string(config.filename)?;

    println!("with text: \n{}", contents);
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
