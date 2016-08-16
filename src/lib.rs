extern crate rustc_serialize;
extern crate docopt;
extern crate regex;

#[cfg(test)]
extern crate rand;

pub mod command;
pub mod cli;
pub mod message;
pub mod errors;
pub mod results;

use std::env;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use rustc_serialize::json;
use results::SlateResult;

/// The main Key-Value structure.
pub struct Slate {

    /// Where the file containing the data is.
    pub filepath: PathBuf,
}

impl Default for Slate {

    /// Get a default Slate. It will use a default file
    /// in your home directory, the `.slate` file.
    ///
    /// # Example
    ///
    /// ```
    /// use slate::Slate;
    ///
    /// let slate: Slate = Default::default();
    /// println!("{}", slate.filepath.to_str().unwrap());
    /// //=> $HOME/.slate
    /// ```
    fn default() -> Slate {
        let mut path = match env::home_dir() {
            Some(home) => home,
            None => panic!("No HOME dir found"),
        };
        path.push(".slate");

        Slate { filepath: path }
    }
}

impl Slate {

    /// Set a key with its value.
    ///
    /// # Example
    ///
    /// ```
    /// use slate::Slate;
    /// use std::env;
    ///
    /// // Create a temporal file for
    /// // the example. You can use Default::default();
    /// // to create the Slate and skip this part.
    /// let mut temp = env::temp_dir();
    /// temp.push(".slate");
    ///
    /// let slate: Slate = Slate { filepath: temp };
    /// let key = "foo".to_string();
    /// let value = "bar".to_string();
    ///
    /// match slate.set(&key, &value) {
    ///   Ok(_) => println!("Saved"),
    ///   Err(e) => panic!("{}", e),
    /// };
    /// ```
    pub fn set(&self, key: &String, value: &String) -> SlateResult<()> {
        let mut contents = try!(self.read());

        contents.insert(key.to_owned(), value.to_owned());

        self.write(&contents)
    }

    /// Get the value of a key
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use slate::Slate;
    /// use std::env;
    ///
    /// // Create a temporal file for
    /// // the example. You can use Default::default();
    /// // to create the Slate and skip this part.
    /// let mut temp = env::temp_dir();
    /// temp.push(".slate");
    ///
    /// let slate: Slate = Slate { filepath: temp };
    /// let key = "foo".to_string();
    ///
    /// match slate.get(&key) {
    ///   Ok(value) => println!("{}", value), //=> bar
    ///   Err(e) => panic!("{}", e),
    /// };
    /// ```
    pub fn get(&self, key: &String) -> SlateResult<String> {
        let contents = try!(self.read());

        match contents.get(key) {
            Some(value) => Ok(value.to_string()),
            None => Ok(String::new()),
        }
    }

    /// Remove completely a key with its value.
    ///
    /// # Example
    ///
    /// ```
    /// use slate::Slate;
    /// use std::env;
    ///
    /// // Create a temporal file for
    /// // the example. You can use Default::default();
    /// // to create the Slate and skip this part.
    /// let mut temp = env::temp_dir();
    /// temp.push(".slate");
    ///
    /// let slate: Slate = Slate { filepath: temp };
    /// let key = "foo".to_string();
    ///
    /// match slate.remove(&key) {
    ///   Ok(_) => println!("Key removed"),
    ///   Err(e) => panic!("{}", e),
    /// };
    /// ```
    pub fn remove(&self, key: &String) -> SlateResult<()> {
        let mut contents = try!(self.read());

        contents.remove(key);

        self.write(&contents)
    }

    /// Remove all keys.
    ///
    /// # Example
    ///
    /// ```
    /// use slate::Slate;
    /// use std::env;
    ///
    /// // Create a temporal file for
    /// // the example. You can use Default::default();
    /// // to create the Slate and skip this part.
    /// let mut temp = env::temp_dir();
    /// temp.push(".slate");
    ///
    /// let slate: Slate = Slate { filepath: temp };
    ///
    /// match slate.clear() {
    ///   Ok(_) => println!("Keys removed"),
    ///   Err(e) => panic!("{}", e),
    /// };
    /// ```
    pub fn clear(&self) -> SlateResult<()> {
        let mut contents = try!(self.read());

        contents.clear();

        self.write(&contents)
    }

    /// Rename a key.
    ///
    /// # Example
    ///
    /// ```
    /// use slate::Slate;
    /// use std::env;
    ///
    /// // Create a temporal file for
    /// // the example. You can use Default::default();
    /// // to create the Slate and skip this part.
    /// let mut temp = env::temp_dir();
    /// temp.push(".slate");
    ///
    /// let slate: Slate = Slate { filepath: temp };
    /// let old = "foo".to_string();
    /// let new = "bar".to_string();
    ///
    /// match slate.rename(&old, &new) {
    ///   Ok(_) => println!("Renamed!"),
    ///   Err(e) => panic!("{}", e),
    /// };
    /// ```
    pub fn rename(&self, src: &String, dts: &String) -> SlateResult<()> {
        let value = try!(self.get(src));
        try!(self.set(dts, &value));
        try!(self.remove(src));

        Ok(())
    }

    /// Get a list of all keys.
    ///
    /// # Example
    ///
    /// ```
    /// use slate::Slate;
    /// use std::env;
    ///
    /// // Create a temporal file for
    /// // the example. You can use Default::default();
    /// // to create the Slate and skip this part.
    /// let mut temp = env::temp_dir();
    /// temp.push(".slate");
    ///
    /// let slate: Slate = Slate { filepath: temp };
    /// let list = match slate.list() {
    ///   Ok(all) => all,
    ///   Err(e) => panic!("{}", e),
    /// };
    ///
    /// for key in &list {
    ///   println!("{}", key);
    /// }
    /// ```
    pub fn list(&self) -> SlateResult<Vec<String>> {
        let contents = try!(self.read());

        let mut keys: Vec<_> = contents.keys().collect();
        keys.sort(); // sort needs a mutable instance!!

        let list: Vec<_> = keys.iter().map(|&s| s.clone()).collect();

        Ok(list)
    }

    /// Read the contents of the Slate file.
    fn read(&self) -> SlateResult<HashMap<String, String>> {
        let mut r = match File::open(&self.filepath) {
            Ok(file) => file,
            Err(_) => {
                let empty = HashMap::new();
                match self.write(&empty) {
                    Ok(_) => File::open(&self.filepath).unwrap(),
                    Err(e) => { return Err(e) }
                }
            },
        };


        let mut buffer = String::new();
        try!(r.read_to_string(&mut buffer));

        let contents: HashMap<String, String> = json::decode(&buffer)
            .unwrap_or(HashMap::new());

        Ok(contents)
    }

    /// Write to the Slate file.
    fn write(&self, contents: &HashMap<String, String>) -> SlateResult<()> {
        let encoded = try!(json::encode(&contents));
        let mut f = try!(File::create(&self.filepath));

        try!(f.write_all(encoded.as_bytes()));

        Ok(())
    }
}


/// Get the version of the library.
pub fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );

    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) =>
            format!("{}.{}.{}", maj, min, pat),
            _ => "".to_string(),
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
        expected.push(".slate");

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
