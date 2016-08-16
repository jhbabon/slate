use cli::parse_args;
use Slate;
use message::Message;
use results::CommandResult;

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

pub fn run(argv: &Vec<String>) -> CommandResult {
    let _args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    let list = try!(slate.list());
    let output = list.join("\n");

    Ok(Some(Message::Info(output)))
}
