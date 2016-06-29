use cli::parse_args;
use Slate;
use message::Message;

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

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_old: String,
    arg_new: String,
}

pub fn run(argv: &Vec<String>) -> Result<Option<Message>, Message> {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    match slate.rename(&args.arg_old, &args.arg_new) {
        Err(e) => Err(Message::Info(e.to_owned())),
        Ok(_) => Ok(Some(Message::Info("The key has been renamed".to_owned()))),
    }
}
