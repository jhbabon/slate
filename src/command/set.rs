use std::process;
use std::io::{self, Read};
use cli::parse_args;
use Slate;

const USAGE: &'static str = "
Slate.

Usage:
  slate set <key> [<value>]

If <value> is not present, <stdin> will be used.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
    arg_value: Option<String>,
}

pub fn run(argv: &Vec<String>) {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());

    let slate: Slate = Default::default();
    let key = args.arg_key;
    let value = match args.arg_value {
        Some(v) => v,
        None => input(),
    };

    match slate.set(&key, &value) {
        Ok(_) => process::exit(0),
        Err(e) => panic!("{}", e),
    };
}

fn input() -> String {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => buffer,
        Err(e) => panic!("There was a problem reading from STDIN: {}", e),
    }
}
