fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    //マクロは引数について呼び出すべきメソッドを自動で判断し、コードを返す。
    println!("(a + b) + (c + d) = {}", e);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}