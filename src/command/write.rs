use docopt::Docopt;
use utils;

const USAGE: &'static str = "
Slate.

Usage:
  slate write <key> [<value>]

If <value> is not present, <stdin> will be used.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: Vec<String>,
    arg_value: Vec<String>,
}

pub fn run(argv: Vec<String>) {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.argv(argv)
                                           .version(Some(utils::version()))
                                           .decode())
                            .unwrap_or_else(|e| e.exit());

    println!("Args: {:?}", args);
}
