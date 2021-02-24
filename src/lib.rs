use std::error::Error;
use std::fs;
use std::env;

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

// test fn
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
pub struct Config {
    pub query: String,
    pub filename: String,
    case_sensitive:bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err(
                "Needs more arguments: a string to search for and a filename to search in.",
            );
        }

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // make case insensitive if add -i after the initial 2 arguments.
        if args.len() == 4  && args[3] == "-i" {

            case_sensitive = false;
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        
        
        Ok(Config { 
            query, 
            filename, 
            case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
    search(&config.query, &contents) 
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}
