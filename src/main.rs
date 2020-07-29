use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application failed: {}", e);
        process::exit(1);
    };
}
