use rugrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    rugrep::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });
}
