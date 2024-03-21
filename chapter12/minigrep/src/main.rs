use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //  the first value in the vector is "target/debug/minigrep", which is the name of our binary
    // dbg!(args);
    // it returns the inner value Ok is wrapping
    // if the value is an Err value, this method calls the code in the closure
    let config = Config::build(&args).unwrap_or_else(|err| {
        // prints errors to terminal, not to file, even if we run $ cargo run -- to poem.txt > output.txt 
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // we use if let rather than unwrap_or_else to check whether run returns an Err value because the run function doesn’t return a value that we want to unwrap
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}