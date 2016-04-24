extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::process;

const USAGE: &'static str = "
Slate.

Usage:
  slate <command> [<args>...]
  slate (-l | --list)
  slate (-h | --help)
  slate (-v | --version)

Options:
  -l --list      Show list of available commands.
  -h --help      Show this screen.
  -v --version   Show version.

Commands:
    write Write a new key and value.
";

mod command {
    pub mod write {
        pub fn run() {
            println!("write command!");
        }
    }
}

#[derive(Debug, RustcDecodable)]
enum Command {
    Write,
}

impl Command {
    fn run(self) {
        match self {
            Command::Write => { command::write::run() }
        }
    }
}

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_command: Option<Command>,
    flag_help: bool,
    flag_version: bool,
    flag_list: bool,
}

fn version() -> String {
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

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.options_first(true)
                                           .version(Some(version()))
                                           .decode())
                            .unwrap_or_else(|e| e.exit());
    println!("Args: {:?}", args);

    match args.arg_command {
        None => {
            println!("Noop!");
            process::exit(404); // NOTE: use consistent error codes
        },
        Some(command) => { command.run() }
    }
}
