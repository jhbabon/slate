use cli::parse_args;
use Slate;
use message::Message;
use results::CommandResult;
use errors::CommandError;

const USAGE: &'static str = "
Slate: Remove an element.

Usage:
  slate remove ([options] | <key>)

Options:
  -h --help   Show this screen.
  -a --all    Remove all keys.

Examples:
  slate remove --all
  #=> All keys have been removed

  slate remove foo
  #=> The key has been removed
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: Option<String>,
    flag_all: bool,
}

pub fn run(slate: &Slate, argv: &Vec<String>) -> CommandResult {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());

    if args.flag_all {
        try!(slate.clear());
        Ok(Some(Message::Info("All keys have been removed".to_string())))
    } else {
        let key: String = match args.arg_key {
            Some(string) => string,
            None => {
                return Err(CommandError::Argument("You must provide the name of a key".to_string()))
            }
        };

        try!(slate.remove(&key));

        Ok(Some(Message::Info("The key has been removed".to_string())))
    }
}
