extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::env;
use std::process;

mod utils;
mod command;

const USAGE: &'static str = "
Slate.

Usage:
  slate <command> [<args>...]
  slate (-h | --help)
  slate (-v | --version)

Options:
  -h --help      Show this screen.
  -v --version   Show version.

Commands:
    add   Write a new key and value.
    get   Read a key.
    list  List all keys.
";

#[derive(Debug, RustcDecodable)]
enum Command {
    Add,
    Get,
    List,
}

impl Command {
    fn run(self) {
        let argv: Vec<String> = env::args().collect();
        match self {
            Command::Add => { command::add::run(argv) },
            Command::Get => { command::get::run(argv) },
            Command::List => { command::list::run(argv) },
        }
    }
}

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_command: Option<Command>,
    flag_help: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.options_first(true)
                                           .version(Some(utils::version()))
                                           .decode())
                            .unwrap_or_else(|e| e.exit());

    match args.arg_command {
        None => {
            println!("Noop!");
            process::exit(404); // NOTE: use consistent error codes
        },
        Some(command) => { command.run() }
    }
}
