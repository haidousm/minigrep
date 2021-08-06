use minigrep::Args;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_args = Args::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(parsed_args) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
