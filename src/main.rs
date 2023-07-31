use jojogrep::Grep;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut grep: Grep = Grep::new(&args).unwrap();

    grep.print_matches();
}
