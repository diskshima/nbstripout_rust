use std::error::Error;
use std::fs;
use std::io::{self, Read};

use json::{array, object, JsonValue};

pub struct Config {
    pub colab: bool,
    pub execution_count: bool,
    pub filename: Option<String>,
    pub outputs: bool,
    pub textconv: bool,
    pub use_stdin: bool,
    pub whitespace: u16,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = read_content(&config)?;

    let output = process_string(&content, &config)?;

    if config.textconv || config.use_stdin {
        println!("{}", output);
    } else {
        fs::write(config.filename.unwrap(), output)?;
    }

    Ok(())
}

fn read_content(config: &Config) -> Result<String, Box<dyn Error>> {
    let content = if config.use_stdin {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        content
    } else {
        match &config.filename {
            Some(filename) => fs::read_to_string(filename).unwrap(),
            None => return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Filename not specified."))),
        }
    };

    Ok(content)
}

fn process_string(content: &String, config: &Config) -> Result<String, Box<dyn Error>> {
    let json = json::parse(&content)?;

    let json = stripout(json, &config);

    Ok(json::stringify_pretty(json, config.whitespace))
}

fn stripout(mut json: JsonValue, config: &Config) -> JsonValue {
    let cells = &mut json["cells"];

    for cell in cells.members_mut() {
        cell["metadata"] = object! {};

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

    fn process_filename_to_json(config: &Config) -> JsonValue {
        let tmp_fn = config.filename.as_ref().unwrap();
        let content = fs::read_to_string(tmp_fn).unwrap();
        let output = process_string(&content, &config).unwrap();
        json::parse(&output).unwrap()
    }

    fn sample_file() -> Option<String> {
        Some(String::from("sample/fibonacci_colab.ipynb"))
    }

    #[test]
    fn remove_cell_metadata() {
        let config = Config {
            colab: false,
            execution_count: false,
            filename: sample_file(),
            outputs: false,
            textconv: false,
            use_stdin: false,
            whitespace: 1,
        };

        let output_json = process_filename_to_json(&config);

        let cells = &output_json["cells"];

        for cell in cells.members() {
            assert_eq!(cell["metadata"], object! {});
        }
    }

    #[test]
    fn remove_execution_count() {
        let config = Config {
            colab: false,
            execution_count: true,
            filename: sample_file(),
            outputs: false,
            textconv: false,
            use_stdin: false,
            whitespace: 1,
        };

        let output_json = process_filename_to_json(&config);

        let cells = &output_json["cells"];

        for cell in cells.members() {
            assert_eq!(cell["execution_count"], JsonValue::Null);
        }
    }

    #[test]
    fn remove_outputs() {
        let config = Config {
            colab: false,
            execution_count: false,
            filename: sample_file(),
            outputs: true,
            textconv: false,
            use_stdin: false,
            whitespace: 1,
        };

        let output_json = process_filename_to_json(&config);

        let cells = &output_json["cells"];

        for cell in cells.members() {
            assert_eq!(cell["outputs"], array! {});
        }
    }

    #[test]
    fn remove_colab() {
        let config = Config {
            colab: true,
            execution_count: false,
            filename: sample_file(),
            outputs: false,
            textconv: false,
            use_stdin: false,
            whitespace: 1,
        };

        let output_json = process_filename_to_json(&config);

        assert!(output_json["metadata"]["colab"].is_null());
        assert!(output_json["metadata"]["accelerator"].is_null());
    }
}
