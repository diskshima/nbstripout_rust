use std::error::Error;
use std::fs;

use json::array;
use json::JsonValue;
use json::object;

pub struct Config {
    pub colab: bool,
    pub execution_count: bool,
    pub filename: String,
    pub outputs: bool,
    pub textconv: bool,
    pub whitespace: u16,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    let output = process_string(&content, &config)?;

    if config.textconv {
        println!("{}", output);
    } else {
        fs::write(config.filename, output)?;
    }

    Ok(())
}

fn process_string(content: &String, config: &Config) -> Result<String, Box<dyn Error>> {
    let json = json::parse(&content)?;

    let json = stripout(json, &config);

    Ok(json::stringify_pretty(json, config.whitespace))
}

fn stripout(mut json: JsonValue, config: &Config) -> JsonValue {
    let cells = &mut json["cells"];

    for cell in cells.members_mut() {
        cell["metadata"] = object!{};

        if config.outputs {
            cell["outputs"] = array![];
        }

        if config.execution_count {
            cell["execution_count"] = JsonValue::Null;
        }
    }

    let metadata = &mut json["metadata"];

    if config.colab {
        metadata.remove("accelerator");
        metadata.remove("colab");
    }

    json
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
    }
}
