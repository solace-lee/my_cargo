use my_cargo::{run, Config};
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    println!("参数为：{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(2);
    };
}
