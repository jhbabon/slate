extern crate rustc_serialize;
extern crate docopt;
extern crate exec;

#[cfg(test)]
extern crate rand;

pub mod command;
pub mod cli;

use std::env;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use rustc_serialize::json;

pub struct Slate {
    pub filepath: PathBuf,
}

impl Default for Slate {
    fn default() -> Slate {
        let mut path = match env::home_dir() {
            Some(home) => home,
            None => panic!("No HOME dir found"), // TODO: What to do here?
        };
        path.push(".slate.json");

        Slate { filepath: path }
    }
}

impl Slate {
    pub fn set(&self, key: &String, value: &String) -> Result<(), &'static str> {
        let mut contents = match self.read() {
            Ok(contents) => contents,
            Err(e) => { return Err(e) },
        };

        contents.insert(key.to_owned(), value.to_owned());

        self.write(&contents)
    }

    pub fn get(&self, key: &String) -> Result<String, &'static str> {
        let contents = match self.read() {
            Ok(contents) => contents,
            Err(e) => { return Err(e) },
        };

        match contents.get(key) {
            Some(value) => Ok(value.to_string()),
            None => Ok(String::new()),
        }
    }

    pub fn remove(&self, key: &String) -> Result<(), &'static str> {
        let mut contents = match self.read() {
            Ok(contents) => contents,
            Err(e) => { return Err(e) },
        };

        contents.remove(key);

        self.write(&contents)
    }

    pub fn clear(&self) -> Result<(), &'static str> {
        let mut contents = match self.read() {
            Ok(contents) => contents,
            Err(e) => { return Err(e) },
        };

        contents.clear();

        self.write(&contents)
    }

    pub fn rename(&self, src: &String, dts: &String) -> Result<(), &'static str> {
        let value = match self.get(src) {
            Ok(v) => v,
            Err(e) => { return Err(e) },
        };

        if let Err(e) = self.set(dts, &value) {
            return Err(e);
        };

        if let Err(e) = self.remove(src) {
            return Err(e);
        };

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<String>, &'static str> {
        let contents = match self.read() {
            Ok(contents) => contents,
            Err(e) => { return Err(e) },
        };

        let mut keys: Vec<_> = contents.keys().collect();
        keys.sort(); // sort needs a mutable instance!!

        let list: Vec<_> = keys.iter().map(|&s| s.clone()).collect();

        Ok(list)
    }

    fn read(&self) -> Result<HashMap<String, String>, &'static str> {
        let mut r = match File::open(&self.filepath) {
            Ok(file) => file,
            Err(_) => { return Err("Cannot open file") }, // control when the file does not exist
        };

        let mut buffer = String::new();
        if let Err(_) = r.read_to_string(&mut buffer) {
            return Err("Error reading file");
        };

        let contents: HashMap<String, String> = match json::decode(&buffer) {
            Ok(hash) => hash,
            Err(_) => HashMap::new(),
        };

        Ok(contents)
    }

    fn write(&self, contents: &HashMap<String, String>) -> Result<(), &'static str> {
        let encoded = json::encode(&contents).unwrap();

        let mut f = match File::create(&self.filepath) {
            Ok(file) => file,
            Err(_) => { return Err("Cannot create file") },
        };
        match f.write_all(encoded.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err("Couldn't save file"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;
    use std::io::prelude::*;
    use std::fs::File;
    use rand::{thread_rng, Rng};

    fn create_temp_file(body: &str) -> PathBuf {
        let random_name: String = thread_rng().gen_ascii_chars().take(10).collect();
        let random_name = random_name + ".json";

        let mut temp = env::temp_dir();
        temp.push(&random_name);

        let mut file = match File::create(&temp) {
            Ok(file) => file,
            Err(e) => panic!("Cannot create temporal file for tests: {:?}", e),
        };
        if let Err(e) = file.write_all(body.as_bytes()) {
            panic!("Cannot add data to temporal file for tests: {:?}", e);
        };

        temp
    }

    #[test]
    fn test_default_slate() {
        let slate: Slate = Default::default();
        let mut expected: PathBuf = env::home_dir().unwrap();
        expected.push(".slate.json");

        assert_eq!(expected, slate.filepath);
    }

    #[test]
    fn it_sets_keys_with_values() {
        let temp = create_temp_file("");
        let mut file = File::open(&temp).unwrap();
        let slate = Slate { filepath: temp };
        let key = "test".to_string();
        let value = "expected".to_string();

        if let Err(e) = slate.set(&key, &value) {
            panic!("Cannot set a value: {:?}", e);
        };

        let mut buffer = String::new();
        let expected = "{\"test\":\"expected\"}";
        if let Err(e) = file.read_to_string(&mut buffer) {
            panic!("Cannot read temporal file for tests: {:?}", e);
        };
        assert_eq!(expected, buffer);
    }

    #[test]
    fn it_gets_keys() {
        let temp = create_temp_file("{\"test\":\"expected\"}");
        let slate = Slate { filepath: temp };
        let key = "test".to_string();

        match slate.get(&key) {
            Ok(value) => assert_eq!("expected", value),
            Err(e) => panic!("Cannot get a value from slate: {:?}", e),
        }
    }

    #[test]
    fn it_gets_missing_keys() {
        let temp = create_temp_file("{\"test\":\"expected\"}");
        let slate = Slate { filepath: temp };
        let key = "missing".to_string();

        match slate.get(&key) {
            Ok(value) => assert_eq!("", value),
            Err(e) => panic!("Cannot get a value from slate: {:?}", e),
        }
    }

    #[test]
    fn it_lists_keys() {
        let temp = create_temp_file("{\"a\":\"1\",\"b\":\"2\"}");
        let slate = Slate { filepath: temp };

        match slate.list() {
            Ok(list) => assert_eq!(vec!["a", "b"], list),
            Err(e) => panic!("Cannot get list of values: {:?}", e),
        }
    }

    #[test]
    fn it_removes_keys() {
        let temp = create_temp_file("{\"test\":\"expected\"}");
        let mut file = File::open(&temp).unwrap();
        let slate = Slate { filepath: temp };
        let key = "test".to_string();

        if let Err(e) = slate.remove(&key) {
            panic!("Cannot remove the key: {:?}", e);
        };

        let mut buffer = String::new();
        let expected = "{}";
        if let Err(e) = file.read_to_string(&mut buffer) {
            panic!("Cannot read temporal file for tests: {:?}", e);
        };
        assert_eq!(expected, buffer);
    }

    #[test]
    fn it_renames_keys() {
        let temp = create_temp_file("{\"test\":\"expected\"}");
        let mut file = File::open(&temp).unwrap();
        let slate = Slate { filepath: temp };
        let key = "test".to_string();
        let new_key = "spec".to_string();

        if let Err(e) = slate.rename(&key, &new_key) {
            panic!("Cannot move the key: {:?}", e);
        };

        let mut buffer = String::new();
        let expected = "{\"spec\":\"expected\"}";
        if let Err(e) = file.read_to_string(&mut buffer) {
            panic!("Cannot read temporal file for tests: {:?}", e);
        };
        assert_eq!(expected, buffer);
    }

    #[test]
    fn it_clears_keys() {
        let temp = create_temp_file("{\"test\":\"expected\"}");
        let mut file = File::open(&temp).unwrap();
        let slate = Slate { filepath: temp };

        if let Err(e) = slate.clear() {
            panic!("Cannot clear keys: {:?}", e);
        };

        let mut buffer = String::new();
        let expected = "{}";
        if let Err(e) = file.read_to_string(&mut buffer) {
            panic!("Cannot read temporal file for tests: {:?}", e);
        };
        assert_eq!(expected, buffer);
    }
}
