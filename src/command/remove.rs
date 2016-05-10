use cli::parse_args;
use Slate;

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

pub fn run(argv: &Vec<String>) -> Result<Option<String>, &str> {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    if args.flag_all {
        match slate.clear() {
            Err(e) => Err(e),
            Ok(_) => Ok(Some("All keys have been removed".to_string())),
        }
    } else {
        let key: String = match args.arg_key {
            Some(string) => string,
            None => { return Err("You must provide the name of a key") },
        };

        match slate.remove(&key) {
            Err(e) => Err(e),
            Ok(_) => Ok(Some("The key has been removed".to_string())),
        }
    }
}
