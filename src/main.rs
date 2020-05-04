use std::process;

use clap::{Arg, App};

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
   let matches = App::new("nbstripout-rust")
                          .version("0.1.0")
                          .author("Daisuke Shimamoto <diskshima@gmail.com>")
                          .about("nbstripout implemented in Rust")
                          .arg(Arg::with_name("colab")
                               .short('c')
                               .long("colab")
                               .about("Strip colab"))
                          .arg(Arg::with_name("execution_count")
                               .short('e')
                               .long("execution-count")
                               .about("Strip execution_count"))
                          .arg(Arg::with_name("outputs")
                               .short('o')
                               .long("outputs")
                               .about("Strip outputs"))
                          .arg(Arg::with_name("whitespace")
                               .short('w')
                               .long("whitespace")
                               .about("Set number of whitespaces for idents")
                               .takes_value(true))
                          .arg(Arg::with_name("input_file")
                               .about("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .get_matches();

    let colab = matches.is_present("colab");
    let execution_count = matches.is_present("execution_count");
    let outputs = matches.is_present("outputs");
    let filename = matches.value_of("input_file").unwrap().to_string();
    let whitespace: u16 = matches.value_of("whitespace")
        .unwrap_or("1").parse().unwrap();

    Ok(Config { colab, execution_count, filename, outputs, whitespace })
}
