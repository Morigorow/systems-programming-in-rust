
#[derive(Debug)]    //println!マクロでenum Cerealを出力するために必要


enum Cereal {   //enum(列挙型)は固定数の有効値をもつ
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![]; //Cerealの空Vectorを初期化
    grains.push(Cereal::Rye);   //要素を１つ追加
//    drop(grains);               //grainsと、その内容を削除
    println!("{:?}", grains);   //削除された値にアクセス＝エラー
}
