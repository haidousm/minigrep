use minigrep::Args;
use std::env;
use std::process;

fn main() {
    let parsed_args = Args::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(parsed_args) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
