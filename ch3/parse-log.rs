#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;      // 便宜上の型名
fn parse_log(line: &str) -> (Event, Message) {
    let parts: Vec<_> = line        // Vec<_>で要素の型をRustに推測させる
                            .splitn(2, ' ')
                            .collect();     // line.splitn()からVec<T>を返す
    if parts.len() == 1 {   // line.splitn()がログを２つに分割できない場合
        return(Event::Unkown, String::from(line))
    }
    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),      // eventの型を認識できなければ行全体を返す
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
                UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
                DELETE 342:LO/22111";
    
    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}