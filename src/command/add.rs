use std::process;
use cli::parse_args;
use Slate;

const USAGE: &'static str = "
Slate.

Usage:
  slate add <key> [<value>]

If <value> is not present, <stdin> will be used.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
    arg_value: String,
}

pub fn run(argv: &Vec<String>) {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());

    let slate: Slate = Default::default();

    // TODO: Set value from stdin if arg_value is empty.
    match slate.add(args.arg_key, args.arg_value) {
        Ok(_) => process::exit(0),
        Err(e) => panic!("{}", e),
    };
}
