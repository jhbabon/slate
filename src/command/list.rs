// use cli::parse_args;
use Slate;

// TODO: Return Result so the main program can show messages
// and errors
pub fn run(_argv: &Vec<String>) {
    let slate: Slate = Slate::new();

    slate.list();
}
