use cli::parse_args;
use Slate;

const USAGE: &'static str = "
Slate: List all value names.

Usage:
  slate list [--help]

Options:
  -h --help  Show this help.

Examples:

  slate list
  #=> foo
  #=> more
";

#[derive(Debug, RustcDecodable)]
struct Args;

pub fn run(argv: &Vec<String>) -> Result<Option<String>, &str> {
    let _args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    let output = match slate.list() {
        Ok(list) => list.join("\n"),
        Err(e) => { return Err(e) },
    };

    Ok(Some(output))
}
