use cli::parse_args;
use Slate;

const USAGE: &'static str = "
Slate: Get a value by name.

Usage:
  slate get <key>
  slate get [options]

Options:
  -h --help  Show this help.

Examples:

  slate get foo
  #=> bar
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
}

pub fn run(argv: &Vec<String>) -> Result<Option<String>, &str> {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    match slate.get(&args.arg_key) {
        Err(e) => Err(e),
        Ok(value) => Ok(Some(value.trim_right().to_string())),
    }
}
