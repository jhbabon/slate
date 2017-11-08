use cli::parse_args;
use Slate;
use results::CommandResult;
use message::Message;

const USAGE: &'static str = "
Slate: Get a value by name.

Usage:
  slate get <key>
  slate get -n <key>
  slate get -h

Options:
  -h --help    Show this help.
  -n --no-eol  Do not print the trailing newline character. Show the value as it was saved.

Examples:

  $ slate get foo
  bar
  $ _

  slate get -n foo
  bar%
  $ _ # no EOL
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_key: String,
    flag_no_eol: bool,
}

pub fn run(slate: &Slate, argv: &Vec<String>) -> CommandResult {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());

    let value = try!(slate.get(&args.arg_key));
    let message: Message = if args.flag_no_eol {
        Message::Raw(value)
    } else {
        Message::Info(value)
    };

    Ok(Some(message))
}
