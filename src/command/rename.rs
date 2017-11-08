use cli::parse_args;
use Slate;
use message::Message;
use results::CommandResult;

const USAGE: &'static str = "
Slate: Rename a key with new name.

Usage:
  slate rename <old> <new>
  slate rename [options]

Options:
  -h --help  Show this help.

Examples:

  slate rename foo bar
  #=> The key has been renamed
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_old: String,
    arg_new: String,
}

pub fn run(slate: &Slate, argv: &Vec<String>) -> CommandResult {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());

    try!(slate.rename(&args.arg_old, &args.arg_new));

    Ok(Some(Message::Info("The key has been renamed".to_string())))
}
