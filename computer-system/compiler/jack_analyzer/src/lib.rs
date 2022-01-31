use std::fs::File;

mod compilation_engine;
mod jack_tokenizer;
use getopts::Matches;

#[derive(Debug)]
pub struct Config {
    filename: String,
    output: Option<String>,
    is_tokens: bool,
}

impl Config {
    pub fn new(matches: &Matches) -> Result<Config, &'static str> {
        if matches.free.len() < 1 {
            return Err("not enough arguments");
        }
        let filename = matches.free[0].clone();
        let output = matches.opt_str("o");
        let is_tokens = matches.opt_present("t");

        Ok(Config {
            filename,
            output,
            is_tokens,
        })
    }
}

pub fn run(config: &Config) {
    let file = File::open(&config.filename).expect("failed to open");
    let mut tokenizer = jack_tokenizer::tokenize(&file);

    if config.is_tokens {
        let xml = tokenizer.to_xml();
        println!("{}", xml);
        return;
    }
    compilation_engine::compile(&mut tokenizer)
}
