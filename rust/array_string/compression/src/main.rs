use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("引数に文字列を渡してください");
        return
    }

    let str = &args[1];

    let result = compress(str);

    if result.len() >= str.len() {
        println!("{}", str);
    } else {
        println!("{}", result);
    }

}

fn compress(str: &str) -> String {
    let mut buf: String = String::new();

    let chars: Vec<char> = str.chars().collect();
    let mut cnt = 0;
    for i in 0..chars.len() {
        cnt += 1;
        if i+1 < chars.len() && chars[i] != chars[i+1] {
            buf.push_str(format!("{}{}", &chars[i], cnt).as_str());
            cnt = 0;
        }
    }
    if cnt > 0 {
        buf.push_str(format!("{}{}", chars.last().unwrap(), cnt).as_str());
    }
    buf
}
