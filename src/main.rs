use std::env;
use std::process;
use rugrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    rugrep::run(config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });
}
