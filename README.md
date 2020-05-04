# nbstripout_rust

nbstripout implementation in Rust.

## Usage

```bash
$ nbstripout_rust --help
nbstripout-rust 0.1.0
Daisuke Shimamoto <diskshima@gmail.com>
nbstripout implemented in Rust

USAGE:
    nbstripout_rust [FLAGS] [OPTIONS] <input_file>

ARGS:
    <input_file>    Sets the input file to use

FLAGS:
    -c, --colab              Strip colab
    -e, --execution-count    Strip execution_count
    -h, --help               Prints help information
    -o, --outputs            Strip outputs
    -t, --textconv           Output to standard out instead of overwriting the file
    -V, --version            Prints version information

OPTIONS:
    -w, --whitespace <whitespace>    Set number of whitespaces for idents
```

## Development

```bash
$ cargo build
```
