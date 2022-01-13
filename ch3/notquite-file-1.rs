#[allow(unused_variables)]  // コンパイラの警告レベルを緩和


type File = String;     // コンパイラはStringとFileを区別しないが、ソースコードは区別する必要がある

fn open(f: &mut File) -> bool {
    true    // この関数は常に成功する
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)] // 一度も使わない関数への警告を緩和
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {     // ！：この関数は消してリターンしないとコンパイラに知らせる
    unimplemented!()    // 到達したらプログラムをクラッシュさせる未実装マクロ
}

fn main() {
    let mut f1 = File::from("f1.txt");  // File型はString型の全てのメソッドを継承している
    open(&mut f1);
    // read(f1, vec![]);
    close(&mut f1);
}