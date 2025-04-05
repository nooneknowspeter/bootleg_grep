use bootleg_grep;
use std::env;
use std::process;

fn main() {
    // handle input arguments from cli
    let args: Vec<String> = env::args().collect();

    let config: bootleg_grep::Config =
        bootleg_grep::Config::configure(&args).unwrap_or_else(|error| {
            println!("Error: {error}");
            process::exit(1);
        });

    // exception handling for file not found
    if let Err(error) = bootleg_grep::run(config) {
        println!("Error: {error}");
        process::exit(1);
    }
}
