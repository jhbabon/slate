use std::env;
use std::path::PathBuf;

const SLATE_FILEPATH: &'static str = "SLATE_FILEPATH";

pub trait EnvWrapper {
    fn var(&self, var: &'static str) -> Result<String, env::VarError>;
}

struct Env;

impl EnvWrapper for Env {
    fn var(&self, var: &'static str) -> Result<String, env::VarError> {
        env::var(var)
    }
}

/// Main Configuration struct
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
    /// let config = Config::from_env();
    /// println!("{}", config.filepath.to_str().unwrap());
    /// //=> /tmp/slate
    /// env::remove_var("SLATE_FILEPATH");
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
    /// let config = Config::from_env();
    /// println!("{}", config.filepath.to_str().unwrap());
    /// //=> $HOME/.slate
    /// ```
    pub fn from_env() -> Config {
        let wrapper = Env;
        Self::new(wrapper)
    }

    pub fn new<T: EnvWrapper>(wrapper: T) -> Config {
        let config: Config = match wrapper.var(SLATE_FILEPATH) {
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

    struct MockEnv {
        value: Option<bool>,
    }

    impl EnvWrapper for MockEnv {
        fn var(&self, _var: &'static str) -> Result<String, env::VarError> {
            match self.value {
                Some(_) => Ok("/tmp/slate".to_string()),
                None => Err(env::VarError::NotPresent)
            }
        }
    }

    #[test]
    fn it_uses_the_homedir_as_default_path() {
        let wrapper = MockEnv { value: None };
        let config: Config = Config::new(wrapper);
        let mut expected: PathBuf = env::home_dir().unwrap();
        expected.push(".slate");

        assert_eq!(expected, config.filepath);
    }

    #[test]
    fn it_uses_the_environment_var_if_set() {
        let wrapper = MockEnv { value: Some(true) };
        let config: Config = Config::new(wrapper);
        let expected: PathBuf = PathBuf::from("/tmp/slate");

        assert_eq!(expected, config.filepath);
    }
}
