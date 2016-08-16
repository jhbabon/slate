use cli::parse_args;
use std::process::Command;
use Slate;
use results::CommandResult;

const USAGE: &'static str = "
Slate: Execute a key as a normal shell command.

Usage:
  slate exec <key>
  slate exec [options]

Options:
  -h --help  Show this help.

Examples:

  slate set echo 'echo hello'
  slate exec echo
  #=> hello
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
}

pub fn run(argv: &Vec<String>) -> CommandResult {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    let value = try!(slate.get(&args.arg_key));
    let value = value.trim_right().to_string();

    let args_list: Vec<&str> = value.split(" ").skip(1).collect();
    let cmd: String = value.split(" ").take(1).collect();

    let mut runner = Command::new(&cmd);
    runner.args(&args_list);

    let mut child = try!(runner.spawn());
    try!(child.wait());

    Ok(None)
}
