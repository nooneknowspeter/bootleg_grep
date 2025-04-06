use bg;
use colored::Colorize;
use std::env;
use std::process;

fn main() {
    // handle input arguments from cli
    let args: Vec<String> = env::args().collect();

    let config: bg::Config = bg::Config::configure(&args).unwrap_or_else(|error| {
        println!("{}", error.to_string().bold().red());
        process::exit(1);
    });

    // exception handling for file not found
    if let Err(error) = bg::run(config) {
        println!("{}", error.to_string().bold().red());
        process::exit(1);
    }
}
