use cli::parse_args;
use exec;
use Slate;

const USAGE: &'static str = "
Slate.

Usage:
  slate exec <key>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
}

pub fn run(argv: &Vec<String>) {
    let args: Args = parse_args(USAGE, argv).unwrap_or_else(|e| e.exit());
    let slate: Slate = Default::default();

    let value = match slate.get(&args.arg_key) {
        Err(e) => panic!("{}", e),
        Ok(value) => value.trim_right().to_owned(),
    };

    let args_list: Vec<&str> = value.split(" ").skip(1).collect();
    let cmd: String = value.split(" ").take(1).collect();

    let mut runner = exec::Command::new(&cmd);
    runner.args(&args_list);
    let err = runner.exec();

    panic!("{}", err);
}
