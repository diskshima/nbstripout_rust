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

    let json = stripout(json);

    // TODO: Add option to configure whitespaces.
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

    let metadata = &mut json["metadata"];

    metadata.remove("accelerator");
    metadata.remove("colab");

    json
}
