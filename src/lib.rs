use std::error::Error;
use std::fs;

use json::array;
use json::JsonValue;
use json::object;

pub struct Config {
    pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let json = json::parse(&content)?;

    // TODO: Stripout elements.
    let json = stripout(json);

    println!("{}", json::stringify_pretty(json, 1));

    // TODO: Add option to save to file.

    Ok(())
}

fn stripout(mut json: JsonValue) -> JsonValue {
    let cells = &mut json["cells"];

    for cell in cells.members_mut() {
        cell["outputs"] = array![];
        cell["metadata"] = object!{};
        cell["execution_count"] = JsonValue::Null;
    }

    json
    // TODO: Strip Google Colab
    // TODO: Strip Google Colab accelerator
}
