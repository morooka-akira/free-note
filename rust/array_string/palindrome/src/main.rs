use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];

    if check_palindrome2(input) {
        println!("これは回文です")
    } else {
        println!("これは回文ではありません")
    }
}

// 同じ数字が奇数個の数を数え 1 or 0 なら回文
fn check_palindrome(str: &str) -> bool {
    let mut char_counter: [u32; 128] = [0; 128];
    for c in str.chars() {
        if c == ' ' {
            continue;
        }
        if c.is_uppercase() {
            char_counter[(c as usize) + 32] += 1;
        } else {
            char_counter[(c as usize)] += 1;
        }
    }

    let mut odd = 0;
    for cnt in char_counter {
        if cnt % 2 == 1 {
            odd += 1
        }
    }

    odd == 1 || odd == 0
}

// アルファベット限定
// 各文字が1回出現したらビットを1立てる(2回出現したら0に戻る)
// 最終的にできたbitに対して - 1をして(奇数1回まで許容分) 0と一致すれば回文
fn check_palindrome2(str: &str) -> bool {
    let mut bit_vector = 0;
    for c in str.chars() {
        if c == ' ' {
            continue;
        }
        let mut index = c as usize;
        if c.is_uppercase() {
            index += 32;
        }
        bit_vector = toggle(bit_vector, index - 97);
    }

    fn toggle(mut bit: i32, index: usize) -> i32 {
        let mask: i32 = 1 << index;
        if bit & mask == 0 {
            bit |= mask;
        } else {
            bit &= !mask;
        }
        bit
    }
    ((bit_vector - 1) & bit_vector) == 0
}
