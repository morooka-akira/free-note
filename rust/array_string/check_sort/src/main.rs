use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let str1 = &args[1];
    let str2 = &args[2];

    if is_sort_str2(str1, str2) {
        println!("並び替えられた文字列です");
    } else {
        println!("並び替えられた文字列ではありません");
    }
}

fn is_sort_str(str1: &str, str2: &str) -> bool {
    if str1 == str2 {
        return true;
    }
    if str1.len() != str2.len() {
        return false;
    }
    for c in str2.chars() {
        if str1.find(c).is_none() {
            return false;
        }
    }
    true
}

// ascii前提
// charの頻出回数を比較する
fn is_sort_str2(str1: &str, str2: &str) -> bool {
    let mut letters: [i32; 128] = [0; 128];
    for c in str1.chars() {
        letters[c as usize] += 1;
    }
    for c in str2.chars() {
        letters[c as usize] -= 1;
        if letters[c as usize] < 0 {
            return false;
        }
    }
    true
}
