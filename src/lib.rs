use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let json = json::parse(&content);

    // TODO: Stripout elements.

    // TODO: Print (for now).
    println!("{:?}", json);

    Ok(())
}
