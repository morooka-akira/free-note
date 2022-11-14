use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let str = &args[1];
    let mut buf: String = String::new();
    for c in str.chars() {
        if c == ' ' {
            buf.push_str("%20")
        } else {
            buf.push(c)
        }
    }
    println!("{}", buf)
}
