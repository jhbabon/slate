use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use rustc_serialize::json;
// use docopt::Docopt;
// use utils;

// const USAGE: &'static str = "
// Slate.

// Usage:
//   slate list
// ";

// #[derive(Debug, RustcDecodable)]
// struct Args {
// }

// TODO: Return Result so the main program can show messages
// and errors
pub fn run(_argv: Vec<String>) {
    // let args: Args = Docopt::new(USAGE)
    //                         .and_then(|d| d.argv(argv)
    //                                        .version(Some(utils::version()))
    //                                        .decode())
    //                         .unwrap_or_else(|e| e.exit());

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

    let slate: HashMap<String, String> = match json::decode(&buffer) {
        Ok(hash) => hash,
        Err(_) => HashMap::new(),
    };

    let mut keys: Vec<_> = slate.keys().collect();
    keys.sort(); // sort needs a mutable instance!!

    for key in &keys {
        println!("{}", key);
    }
}
