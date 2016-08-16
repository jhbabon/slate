use regex::{Regex, NoExpand};
use cli::parse_args;
use Slate;
use message::Message;
use results::CommandResult;

const USAGE: &'static str = "
Slate: Use a key as a snippet and replace its values with
whatever you want.

A snippet is any text with placeholders with the form `:name:`.
To replace these placeholders you just need to pass the name
of the placeholder followed by a new value.

Usage:
  slate snippet <key> --replace (<placeholder> <value>)...
  slate snippet -h | --help

Options:
  -r --replace  Variables and values to change.
  -h --help     Show this help.

Examples:

  slate set dummy 'ssh -1246AaCfGgKkMNnqsTtVvXxYy :user:@:host:'
  slate snippet dummy -r user me host localhost
  #=> ssh -1246AaCfGgKkMNnqsTtVvXxYy me@localhost
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
    arg_placeholder: Vec<String>,
    arg_value: Vec<String>,
}

pub fn run(argv: &Vec<String>) -> CommandResult {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();
    let pairs = args.arg_placeholder.iter()
        .zip(args.arg_value.iter());

    let value = try!(slate.get(&args.arg_key));
    let snippet = value.trim_right().to_string();

    let output: String = pairs.fold(snippet, replacer);

    Ok(Some(Message::Raw(output)))
}

fn replacer(snippet: String, (placeholder, value): (&String, &String)) -> String {
    let token = format!(":{}:", placeholder);
    let re = Regex::new(&token).unwrap();

    re.replace_all(&snippet, NoExpand(value))
}
