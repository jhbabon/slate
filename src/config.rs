use std::env;
use std::path::PathBuf;

const SLATE_FILEPATH: &'static str = "SLATE_FILEPATH";

/// Main Configuratio struct
#[derive(Clone)]
pub struct Config {
    /// Path to the slate file.
    pub filepath: PathBuf,
}

impl Config {
    /// Get a new Config struct with all of its fields
    /// initialized with env vars or default values.
    ///
    /// If the env var `SLATE_FILEPATH` is set with a valid
    /// path to a file, that value will be used when initializing
    /// the Config struct.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slate::config::Config;
    /// use std::env;
    ///
    /// env::set_var("SLATE_FILEPATH", "/tmp/var");
    /// let config = Config::new();
    /// println!("{}", config.filepath.to_str().unwrap());
    /// //=> /tmp/slate
    /// ```
    ///
    /// If there is no env var, the default `$HOME/.slate` value
    /// will be used.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slate::config::Config;
    /// use std::env;
    ///
    /// env::remove_var("SLATE_FILEPATH");
    /// let config = Config::new();
    /// println!("{}", config.filepath.to_str().unwrap());
    /// //=> $HOME/.slate
    /// ```
    pub fn new() -> Config {
        let config: Config = match env::var(SLATE_FILEPATH) {
            Ok(value) => Config { filepath: PathBuf::from(value) },
            Err(_) => Default::default(),
        };

        config
    }
}

impl Default for Config {
    /// Default implementation of a Config struct.
    ///
    /// It will always use the `$HOME/.slate` for the
    /// filepath.
    ///
    /// It panics if there is no HOME dir.
    fn default() -> Config {
        let mut path = match env::home_dir() {
            Some(home) => home,
            None => panic!("No HOME dir found"),
        };
        path.push(".slate");

        Config { filepath: path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn it_uses_the_homedir_as_default_path() {
        let config: Config = Config::new();
        let mut expected: PathBuf = env::home_dir().unwrap();
        expected.push(".slate");

        assert_eq!(expected, config.filepath);
    }

    #[test]
    fn it_uses_the_environment_var_if_set() {
        // setup
        env::set_var("SLATE_FILEPATH", "/tmp/slate");

        let config: Config = Config::new();
        let expected: PathBuf = PathBuf::from("/tmp/slate");

        assert_eq!(expected, config.filepath);

        // teardown
        env::remove_var("SLATE_FILEPATH");
    }
}
