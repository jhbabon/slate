use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use rustc_serialize::json;
use docopt::Docopt;
use utils;

const USAGE: &'static str = "
Slate.

Usage:
  slate get <key>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
}

// TODO: Return Result so the main program can show messages
// and errors
pub fn run(argv: Vec<String>) {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.argv(argv)
                                           .version(Some(utils::version()))
                                           .decode())
                            .unwrap_or_else(|e| e.exit());

    let mut path = match env::home_dir() {
        Some(home) => home,
        None => panic!("No HOME dir found"),
    };
    path.push(".slate.json");

    // read current state
    let mut r = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("Cannot open file"), // control when the file does not exist
    };

    let mut buffer = String::new();
    r.read_to_string(&mut buffer);

    let mut slate: HashMap<String, String> = match json::decode(&buffer) {
        Ok(hash) => hash,
        Err(_) => HashMap::new(),
    };

    match slate.get(&args.arg_key) {
        Some(value) => { println!("{}", value) },
        None => { println!("The key {} doesn't exist", args.arg_key) }
    };
}
