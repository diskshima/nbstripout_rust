use std::env;
use std::process;

use nbstripout_rust;
use nbstripout_rust::Config;

fn main() {
    let config = match config_from_args() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to parse arguments: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = nbstripout_rust::run(config) {
        eprintln!("Errored with: {}", e);

        process::exit(1);
    }
}

fn config_from_args() -> Result<Config, &'static str> {
    let mut args = env::args();

    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file name"),
    };

    Ok(Config { filename })
}
