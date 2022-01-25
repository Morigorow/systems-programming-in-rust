#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat{ id: 0 };
    let sat_b = CubeSat{ id: 1 };
    let sat_c = CubeSat{ id: 2 };

    let a_status = check_status(sat_a); // 所有権がcheck_statusの引数sat_idに移動するためエラー
    let a_status = check_status(sat_a); // 所有権がcheck_statusの引数sat_idに移動するためエラー

    println!("a: {:?}", a_status);
}