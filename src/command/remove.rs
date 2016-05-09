use cli::parse_args;
use Slate;

const USAGE: &'static str = "
Slate.

Usage:
  slate remove ([options] | <key>)

Options:
  -h --help   Show this screen.
  -a --all    Remove all keys.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: Option<String>,
    flag_all: bool,
}

pub fn run(argv: &Vec<String>) {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    if args.flag_all {
        match slate.clear() {
            Err(e) => panic!("{}", e),
            Ok(_) => println!("All keys have been removed."),
        };
    } else {
        let key: String = match args.arg_key {
            Some(string) => string,
            None => panic!("You must provide the name of a key"),
        };

        match slate.remove(&key) {
            Err(e) => panic!("{}", e),
            Ok(_) => println!("The key has been removed."),
        };
    }
}
