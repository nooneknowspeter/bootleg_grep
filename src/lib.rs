use colored::Colorize;
use std::error::Error;
use std::{fs, process};

pub struct Config {
    query: String,
    file_path: String,
    is_sensitive: bool,
}

impl Config {
    // configure structure properties to use in other functions
    pub fn configure(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            println!("\n{}\n", "bg <query> <file> [optional]".bold());
            println!("{}\n", "- bg: executable");
            println!("{}\n", "- <query>: query (no quotation marks)");
            println!("{}\n", "- <file>: ./path/to/file");
            println!("{}", "- [optional]:");
            println!("{}", "    -i -> insensitive search");

            return Err("");
        }

        let query_arg: String = args[1].clone();
        let file_path_arg: String = args[2].clone();
        let is_sensitive_arg: bool = if args.len() > 3 {
            if args[3] != "-i" {
                return Err("Invalid flag");
            }
            false
        } else {
            true
        };

        Ok(Config {
            query: query_arg,
            file_path: file_path_arg,
            is_sensitive: is_sensitive_arg,
        })
    }
}

// use config struct contents to read contents of file
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read contents for file
    let contents = fs::read_to_string(config.file_path).unwrap_or_else(|error| {
        println!("{}", error.to_string().bold().red());
        process::exit(1);
    });

    if !config.is_sensitive {
        // insensitive searching
        for line in search_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        // sensitive searching
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}

// case sensitive searching
fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            // result.push(line);
            result.push(line);
        }
    }

    result
}

// case insensitive searching
fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            // result.push(line);
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_sensitive() {
        let query = "rainbow";
        let contents = "Under the rainbow";

        assert_eq!(
            vec!["Under the rainbow"],
            search(&query, &contents),
            "Checking if query -> \"{query}\" was found in contents -> \"{contents}\""
        );
    }

    #[test]
    fn test_case_insensitive() {
        let query = "rAinBow";
        let contents = "Under the rainbow";

        assert_eq!(
            vec!["Under the rainbow"],
            search_insensitive(&query, &contents),
            "Checking if query -> \"{query}\" was found in contents -> \"{contents}\""
        );
    }
}
