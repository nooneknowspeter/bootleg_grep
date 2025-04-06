pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // configure structure properties to use in other functions
    pub fn configure(args: &[String]) -> Result<Config, &str> {

        let query_arg: String = args[1].clone();
        let file_path_arg: String = args[2].clone();

        Ok(Config {
            query: query_arg,
            file_path: file_path_arg,
        })
    }
}

