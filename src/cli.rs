use rustc_serialize::Decodable;
use docopt;
use std::process;

use command;

const USAGE: &'static str = "
Slate.

Usage:
  slate <command> [<args>...]
  slate [options]

Options:
  -h --help      Show this screen.
  -v --version   Show version.

Commands:
   set     Write a new key and value.
   get     Read a key.
   list    List all keys.
   rename  Rename a key.
   remove  Delete a key.
   exec    Run a key value as a command.
   snippet Get a key and replace all placeholders with new data.
";

#[derive(Debug, RustcDecodable)]
enum Command {
    Set,
    Get,
    List,
    Remove,
    Rename,
    Exec,
    Snippet,
}

impl Command {
    fn run(self, argv: &Vec<String>) -> Result<Option<String>, &str> {
        match self {
            Command::Set => { command::set::run(argv) },
            Command::Get => { command::get::run(argv) },
            Command::List => { command::list::run(argv) },
            Command::Remove => { command::remove::run(argv) },
            Command::Rename => { command::rename::run(argv) },
            Command::Exec => { command::exec::run(argv) },
            Command::Snippet => { command::snippet::run(argv) },
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
    let args: Args = parse_main_args(USAGE, &argv).unwrap_or_else(|e| e.exit());

    let command = match args.arg_command {
        None => {
            println!("Noop!");
            process::exit(404); // NOTE: use consistent error codes
        },
        Some(command) => command,
    };

    match command.run(&argv) {
        Err(e) => error(e),
        Ok(message) => out(message),
    };
}

pub fn parse_args<T>(usage: &str, argv: &Vec<String>) -> Result<T, docopt::Error>
    where T: Decodable {
        docopt::Docopt::new(usage)
            .and_then(|d| d.argv(argv)
                           .version(Some(super::version()))
                           .decode())
}

fn parse_main_args<T>(usage: &str, argv: &Vec<String>) -> Result<T, docopt::Error>
    where T: Decodable {
        docopt::Docopt::new(usage)
            .and_then(|d| d.argv(argv)
                           .options_first(true)
                           .version(Some(super::version()))
                           .decode())
}

fn error(err: &str) {
    println!("{}", err);
    process::exit(1);
}

fn out(message: Option<String>) {
    if let Some(msg) = message {
        println!("{}", msg);
    };
    process::exit(0);
}
