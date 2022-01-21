#![allow(unused_variables)] // 関数内で使わない変数について警告を抑制

#[derive(Debug)]
struct File;    // スタブのFile型を定義

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

fn main() {
    let f = File{};
    let mut buffer = vec![];
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} bytes read from {:?}", n_bytes, f);
}