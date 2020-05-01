use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};


fn main() {
    let stdout  = stdout();
    let out = b"hello Rustaceans";
    let width = 16;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
    println!("Hello, world!");
}
