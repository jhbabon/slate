// use cli::parse_args;
use Slate;

pub fn run(_argv: &Vec<String>) -> Result<String, &'static str> {
    let slate: Slate = Default::default();

    match slate.list() {
        Ok(list) => Ok(list.join("\n")),
        Err(e) => Err(e),
    }
}
