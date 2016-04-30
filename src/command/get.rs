use cli::parse_args;
use Slate;

const USAGE: &'static str = "
Slate.

Usage:
  slate get <key>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
}

pub fn run(argv: &Vec<String>) -> Result<String, &'static str> {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    slate.get(args.arg_key)
}
