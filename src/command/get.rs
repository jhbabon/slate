use cli::parse_args;
use Slate;
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

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
    flag_no_eol: bool,
}

pub fn run(argv: &Vec<String>) -> Result<Option<Message>, Message> {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    match slate.get(&args.arg_key) {
        Ok(value) => {
            let message: Message = if args.flag_no_eol {
                Message::Raw(value)
            } else {
                Message::Info(value)
            };

            Ok(Some(message))
        },
        Err(e) => Err(Message::Info(e.to_owned())),
    }
}
