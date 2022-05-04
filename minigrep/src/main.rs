use std::env;
use std::process;
use minigrep::Config;

fn main() {

    //reads command line arguments and stores in args, a vector of String(s)
    //let args: Vec<String> = env::args().collect();

    //initializes a Config struct with the args in the query and filename fields
    //let config = Config::new(&args).unwrap_or_else(
        // handles errors
    //    |err| {
    //        eprintln!("Problem parsing arguments: {}", err);
    //        process::exit(1)
    //    }
    //);


    //new implementation with iterators
    let config = Config::new(env::args()).unwrap_or_else( |err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });


    // println!("Searching for '{}'", config.query);
    // println!("In file '{}'", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }




}