
use rustc_serialize::Decodable;
use docopt;
use std::process;

use command;

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
    fn run(self, argv: &Vec<String>) -> Result<String, &'static str> {
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

pub fn run(argv: Vec<String>) {
    let args: Args = parse_args(USAGE, &argv).unwrap_or_else(|e| e.exit());

    let command = match args.arg_command {
        None => {
            println!("Noop!");
            process::exit(404); // NOTE: use consistent error codes
        },
        Some(command) => command,
    };

    match command.run(&argv) {
        Err(e) => panic!("{:?}", e),
        Ok(value) => println!("{}", value),
    }
}

pub fn parse_args<T>(usage: &str, argv: &Vec<String>) -> Result<T, docopt::Error>
    where T: Decodable {
        docopt::Docopt::new(usage)
            .and_then(|d| d.argv(argv)
                        .version(Some(super::version()))
                        .decode())
}
