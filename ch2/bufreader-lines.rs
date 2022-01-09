use std::fs::File;
use std::io::BufReader;     // バッファを備えたReader。行単位で読み込む際に効率が良い
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);

    // BufReader::lines()は各行の最後にある改行を取り除く
    for line_ in reader.lines() {
        let line = line_.unwrap();  // Result型をunwrap
        println!("{} ({} bytes long)", line, line.len());
    }
}