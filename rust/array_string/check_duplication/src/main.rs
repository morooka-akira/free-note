use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = &args[1];

    if !is_unique_chars(word) {
        println!("重複あり");
    } else {
        println!("重複なし");
    }
}

fn is_unique_chars(str: &str) -> bool {
    // asciiは128文字しかないため、128以上あれば重複
    if str.len() > 128 {
        return false;
    }
    let mut char_set: [bool; 128] = [false; 128];
    for c in str.chars() {
        if let Some(f) = str.find(c) {
            if char_set[f] {
                return false;
            }
            char_set[f] = true;
        }
    }
    true
}
