use std::fs::File;

mod jack_tokenizer;

pub fn run(file_name: &str) {
    println!("run jack analyzer");
    let file = File::open(file_name).expect("failed to open");
    jack_tokenizer::tokenize(&file);
}
