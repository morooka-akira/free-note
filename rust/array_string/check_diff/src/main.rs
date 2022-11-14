use std::{cmp, env};

fn main() {
    let args: Vec<String> = env::args().collect();

    let str1 = &args[1];
    let str2 = &args[2];
    if check_diff(str1, str2) {
        println!("変換可能です")
    } else {
        println!("変換できません")
    }
}

fn check_diff(str1: &str, str2: &str) -> bool {
    // 文字数が1文字以上違う場合はfalse
    if str1.len().abs_diff(str2.len()) > 1 {
        return false;
    }

    if str1.len() != str2.len() {
        let index = cmp::max(str1.len(), str2.len());
        for i in 0..index {
            if str1.chars().nth(i).is_none() || str2.chars().nth(i).is_none() {
                continue;
            }
            if str1.chars().nth(i).unwrap() != str2.chars().nth(i).unwrap() {
                return false;
            }
        }
        return true;
    }

    let mut diff = 0;
    let index = cmp::max(str1.len(), str2.len());
    for i in 0..index {
        if str1.chars().nth(i).is_none() || str2.chars().nth(i).is_none() {
            continue;
        }
        if str1.chars().nth(i).unwrap() != str2.chars().nth(i).unwrap() {
            diff += 1;
        }
        if diff > 1 {
            return false;
        }
    }
    true
}
