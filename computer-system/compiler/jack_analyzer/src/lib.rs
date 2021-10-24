use std::fs::File;

mod compilation_engine;
mod jack_tokenizer;

pub fn run(file_name: &str) {
    println!("run jack analyzer");
    let file = File::open(file_name).expect("failed to open");

    let tokens = jack_tokenizer::tokenize(&file);
    compilation_engine::compile(tokens)
}
