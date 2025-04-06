use std::error::Error;
use std::{fs, process};

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // configure structure properties to use in other functions
    pub fn configure(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Insufficient arguments");
        }

        let query_arg: String = args[1].clone();
        let file_path_arg: String = args[2].clone();

        Ok(Config {
            query: query_arg,
            file_path: file_path_arg,
        })
    }
}

// use config struct contents to read contents of file
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read contents for file
    let contents = fs::read_to_string(config.file_path).unwrap_or_else(|error| {
        println!("Error parsing file: {error}");
        process::exit(1);
    });

    println!("{contents}");

    Ok(())
}
