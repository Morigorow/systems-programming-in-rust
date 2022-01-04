// カウンタのインクリメントスピード試験
use std::time::{Duration, Instant};

fn main() {
    let mut count = 1;
    let time_limit = Duration::new(1, 0); // 1秒の間隔を表すDurationを作る
    let start = Instant::now();             // システムクロックから開始時間Instantを取得
    // 
    while(Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
}