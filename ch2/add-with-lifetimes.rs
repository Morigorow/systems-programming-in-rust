// 参照を使うときはライフタイムパラメータを付ける
// パラメータiはライフタイムaを持つi32型への参照
fn add_with_lifeteimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

// 同じ型Tの引数を２つ受け取り、その型の値を１つ返す
// T,U,V,E(エラーの型)など採用する文字は任意
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {      // トレイト境界でstd::ops::Addの実装を強制
    i + j   // トレイト境界によってどんな型でも加算が保証される
}

fn main() {
    let a = 10;
    let b = 20;
    // 関数呼び出しでライフタイムの記述は不要
    let res = add_with_lifeteimes(&a, &b);
    println!("{}", res);
}