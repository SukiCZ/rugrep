use std::env;
use std::process;
use rugrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for `{0}` in file `{1}`", config.query, config.file_path);

    rugrep::run(config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });
}
