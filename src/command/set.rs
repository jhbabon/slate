use std::io::{self, Read};
use cli::parse_args;
use Slate;

const USAGE: &'static str = "
Slate: Set a value using a name (or key).

Usage:
  slate set <key> [<value>]
  slate set [options]

If <value> is not present, <stdin> will be used.

Options:
  -h --help  Show this help.

Examples:

  slate set foo bar

  cat config.yml | slate set config
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
    arg_value: Option<String>,
}

pub fn run(argv: &Vec<String>) -> Result<Option<String>, &str> {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());

    let slate: Slate = Default::default();
    let key = args.arg_key;
    let value = match args.arg_value {
        Some(v) => Ok(v),
        None => input(),
    };
    let value = match value {
        Ok(v) => v,
        Err(e) => { return Err(e) },
    };

    match slate.set(&key, &value) {
        Ok(_) => Ok(None),
        Err(e) => Err(e),
    }
}

fn input() -> Result<String, &'static str> {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => Err("There was a problem reading from STDIN"),
    }
}
