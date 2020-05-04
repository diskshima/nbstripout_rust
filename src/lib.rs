use std::error::Error;
use std::fs;

use json::JsonValue;

pub struct Config {
    pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let json = json::parse(&content)?;

    // TODO: Stripout elements.
    let json = stripout(json);

    // TODO: Print (for now).
    println!("{}", json::stringify_pretty(json, 1));

    Ok(())
}

fn stripout(json: JsonValue) -> JsonValue {
    json
}
