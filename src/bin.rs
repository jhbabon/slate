extern crate slate;

use std::env;
use slate::cli;

pub fn main() {
    let argv: Vec<String> = env::args().collect();

    cli::run(argv);
}
