use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive};

/*
Calling the command line parsing logic with the argument values
Setting up any other configuration
Calling a run function in lib.rs
Handling the error if run returns an error
*/


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}\n\n", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    //dbg!(args);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results{
        println!("{line}");
    }

    Ok(())
}

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    
}

/*
Set env var on cmd line like:
$ export IGNORE_CASE=1 cargo run -- to poem.txt
Remove with:
unset IGNORE_CASE
Check with:
echo $IGNORE_CASE
List all env vars
env or printenv
*/

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3{
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path, 
            ignore_case 
        })
    }
}