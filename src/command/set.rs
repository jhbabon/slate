use std::io::{self, Read};
use cli::parse_args;
use Slate;
use results::CommandResult;
use errors::CommandError;

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

#[derive(Debug, Deserialize)]
struct Args {
    arg_key: String,
    arg_value: Option<String>,
}

pub fn run(slate: &Slate, argv: &Vec<String>) -> CommandResult {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());

    let key = args.arg_key;
    let value = match args.arg_value {
        Some(v) => Ok(v),
        None => input(),
    };
    let value = try!(value);
    try!(slate.set(&key, &value));

    Ok(None)
}

fn input() -> Result<String, CommandError> {
    let mut buffer = String::new();
    try!(io::stdin().read_to_string(&mut buffer));

    Ok(buffer)
}
