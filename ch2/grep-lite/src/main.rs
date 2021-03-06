use regex::Regex;   // Regex型をregexクレートからローカルスコープにインポート
use clap::{App,Arg};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;

// 標準入力またはファイルの両方を使えるように抽象化する
fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
        .get_matches();
    
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    // ファイル指定がなければ標準入力とみなし”-”をinputに代入
    let input = args.value_of("input").unwrap_or("_");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

/* 
fn main() {
    let args = App::new("grep-lite")    // コマンド引数のパーサを徐々に構築する
        .version("0.1")                 // 個々の引数がArgを１つ取るが、この場合は１つのみ
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .get_matches();
    // パターン引数を抽出する
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();    // unwrapでResultのラップを解く

    let quote = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),    // Some(T)はOptionの肯定的なケース。re.find()が成功
            None => (),
        }
    }
}
*/