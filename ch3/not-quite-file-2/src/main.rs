#![allow(unused_variables)]     // 警告を抑制

#[derive(Debug)]    // println!マクロと同類のマクロでFileをつかえるようにする
struct File{
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(
    f: &File,
    save_to: &mut Vec<u8>,
) -> usize {                    // 読み込んだバイト数を返す
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);   // 入力データを入れるのに十分なスペースを確保
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let mut f2 = File{
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };
    
    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close (&mut f2);

    let text = String::from_utf8_lossy(&buffer);    // Vec<u8>をStringに変換

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);
}