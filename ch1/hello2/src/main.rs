fn greet_world() {
    println!("Hello, world!");
    //Unicode が標準でサポートされている/UTF-8でエンコード
    let southern_germany: &str = "Grüß Gott!";
    let japanese: &str = "ハロー・ワールド！";
    let regions: [&str; 2] = [southern_germany, japanese];
    //&をつけて借用（borrow）
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}