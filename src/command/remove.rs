use cli::parse_args;
use Slate;
use message::Message;

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

pub fn run(argv: &Vec<String>) -> Result<Option<Message>, Message> {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    if args.flag_all {
        match slate.clear() {
            Err(e) => Err(Message::Info(e.to_owned())),
            Ok(_) => Ok(Some(Message::Info("All keys have been removed".to_owned()))),
        }
    } else {
        let key: String = match args.arg_key {
            Some(string) => string,
            None => { return Err(Message::Info("You must provide the name of a key".to_owned())) },
        };

        match slate.remove(&key) {
            Err(e) => Err(Message::Info(e.to_owned())),
            Ok(_) => Ok(Some(Message::Info("The key has been removed".to_owned()))),
        }
    }
}
