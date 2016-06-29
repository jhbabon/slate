use cli::parse_args;
use exec;
use Slate;
use message::Message;

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

pub fn run(argv: &Vec<String>) -> Result<Option<Message>, Message> {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    let value = match slate.get(&args.arg_key) {
        Err(e) => { return Err(Message::Info(e.to_owned())) },
        Ok(value) => value.trim_right().to_owned(),
    };

    let args_list: Vec<&str> = value.split(" ").skip(1).collect();
    let cmd: String = value.split(" ").take(1).collect();

    let mut runner = exec::Command::new(&cmd);
    runner.args(&args_list);
    let _err = runner.exec();

    // If this line is executed it means that the process
    // didn't change and so there must be an error.
    Err(Message::Info("There was an error executing the command".to_string()))
}
