extern crate rustc_serialize;
extern crate docopt;

pub mod command;
pub mod cli;

use std::env;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use rustc_serialize::json;

pub struct Slate {
    filename: PathBuf,
}

impl Slate {
    pub fn new() -> Slate {
        let mut path = match env::home_dir() {
            Some(home) => home,
            None => panic!("No HOME dir found"),
        };
        path.push(".slate.json");

        Slate {
            filename: path,
        }
    }

    pub fn add(&self, key: String, value: String) {
        let mut r = match File::open(&self.filename) {
            Ok(file) => file,
            Err(_) => panic!("Cannot open file"), // control when the file does not exist
        };

        let mut buffer = String::new();
        if let Err(_) = r.read_to_string(&mut buffer) {
            panic!("Error reading file")
        };

        let mut slate: HashMap<String, String> = match json::decode(&buffer) {
            Ok(hash) => hash,
            Err(_) => HashMap::new(),
        };

        // write new values
        slate.insert(key, value);

        let encoded = json::encode(&slate).unwrap();

        let mut f = match File::create(&self.filename) {
            Ok(file) => file,
            Err(_) => panic!("Cannot create file"),
        };
        match f.write_all(encoded.as_bytes()) {
            Ok(_) => { println!("OK") },
            Err(_) => { panic!("Couldn't save file") }
        };
    }

    pub fn get(&self, key: String) {
        // read current state
        let mut r = match File::open(&self.filename) {
            Ok(file) => file,
            Err(_) => panic!("Cannot open file"), // control when the file does not exist
        };

        let mut buffer = String::new();
        match r.read_to_string(&mut buffer) {
            Ok(_) => false,
            Err(_) => panic!("Error reading file")
        };

        let slate: HashMap<String, String> = match json::decode(&buffer) {
            Ok(hash) => hash,
            Err(_) => HashMap::new(),
        };

        match slate.get(&key) {
            Some(value) => { println!("{}", value) },
            None => { println!("The key {} doesn't exist", key) }
        };
    }

    pub fn list(&self) {
        // read current state
        let mut r = match File::open(&self.filename) {
            Ok(file) => file,
            Err(_) => panic!("Cannot open file"), // control when the file does not exist
        };

        let mut buffer = String::new();
        match r.read_to_string(&mut buffer) {
            Ok(_) => false,
            Err(_) => panic!("Error reading file")
        };

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
}

pub fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );

    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) =>
            format!("{}.{}.{}", maj, min, pat),
            _ => "".to_owned(),
    }
}
