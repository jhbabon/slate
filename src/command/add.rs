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

// TODO: Return Result so the main program can show messages
// and errors
pub fn run(argv: &Vec<String>) {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());

    let slate: Slate = Slate::new();

    // TODO: Set value from stdin if arg_value is empty.
    slate.add(args.arg_key, args.arg_value); // TODO: Return result.
}
