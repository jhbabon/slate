use rustc_serialize::Decodable;
use docopt;
use std::process;

use command;
use errors::CommandError;
use results::CommandResult;
use message::Message;
use config::Config;
use Slate;

const USAGE: &'static str = "
Slate: Manage your snippets from your command line.

Note that Slate will use the file ~/.slate to save
its contents.

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
    fn run(self, slate: &Slate, argv: &Vec<String>) -> CommandResult {
        match self {
            Command::Set => command::set::run(slate, argv),
            Command::Get => command::get::run(slate, argv),
            Command::List => command::list::run(slate, argv),
            Command::Remove => command::remove::run(slate, argv),
            Command::Rename => command::rename::run(slate, argv),
            Command::Exec => command::exec::run(slate, argv),
            Command::Snippet => command::snippet::run(slate, argv),
        }
    }
}

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_command: Command,
    flag_help: bool,
    flag_version: bool,
}

/// Main entry point of the command.
///
/// It executes the given command and handles its output and errors.
pub fn run(argv: Vec<String>) {
    let args: Args = parse_main_args(USAGE, &argv).unwrap_or_else(|e| e.exit());

    let command = args.arg_command;

    let config = Config::new();
    let slate: Slate = From::from(&config);

    match command.run(&slate, &argv) {
        Err(e) => error(e),
        Ok(message) => out(message),
    };
}

/// Parse arguments based on a USAGE slice string.
///
/// This is used mainly by subcommands.
pub fn parse_args<T>(usage: &str, argv: &Vec<String>) -> Result<T, docopt::Error>
    where T: Decodable
{
    docopt::Docopt::new(usage).and_then(|d| {
        d.argv(argv)
            .version(Some(super::version()))
            .decode()
    })
}

/// Parse arguments for the main command.
fn parse_main_args<T>(usage: &str, argv: &Vec<String>) -> Result<T, docopt::Error>
    where T: Decodable
{
    docopt::Docopt::new(usage).and_then(|d| {
        d.argv(argv)
            .options_first(true)
            .version(Some(super::version()))
            .decode()
    })
}

/// Show errors to the user.
fn error(err: CommandError) {
    println!("{}", err);
    process::exit(1);
}

/// Show program messages to the user.
fn out(message: Option<Message>) {
    if let Some(msg) = message {
        print!("{}", msg);
    };
    process::exit(0);
}
