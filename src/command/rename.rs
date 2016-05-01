use cli::parse_args;
use Slate;

const USAGE: &'static str = "
Slate.

Usage:
  slate rename <old> <new>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_old: String,
    arg_new: String,
}

pub fn run(argv: &Vec<String>) {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    match slate.rename(args.arg_old, args.arg_new) {
        Err(e) => panic!("{}", e),
        Ok(_) => println!("The key has been renamed."),
    };
}
