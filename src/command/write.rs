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
  slate write <key> [<value>]

If <value> is not present, <stdin> will be used.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
    arg_value: String,
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
        Err(_) => panic!("Cannot open file"),
    };

    let mut buffer = String::new();
    r.read_to_string(&mut buffer);

    let mut slate: HashMap<String, String> = json::decode(&buffer).unwrap();

    // write new values
    slate.insert(args.arg_key, args.arg_value);

    let encoded = json::encode(&slate).unwrap();

    let mut f = match File::create(&path) {
        Ok(file) => file,
        Err(_) => panic!("Cannot create file"),
    };
    match f.write_all(encoded.as_bytes()) {
        Ok(_) => { println!("OK") },
        Err(_) => { panic!("Couldn't save file") }
    };
}
