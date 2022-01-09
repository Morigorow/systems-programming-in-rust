use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);
    // １個のStringオブジェクトをプログラムの存続期間を通じて再利用する
    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break
        }
        println!("{} ({} bytes long)", line, len);
        line.truncate(0);   // Stringの長さを０に縮めて、前の行の内容が残ってしまうことを防ぐ
    }
}