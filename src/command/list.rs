// use cli::parse_args;
use Slate;

pub fn run(_argv: &Vec<String>) {
    let slate: Slate = Default::default();

    let list = match slate.list() {
        Ok(list) => list,
        Err(e) => panic!("{}", e),
    };

    for key in &list {
        println!("{}", key);
    };
}
