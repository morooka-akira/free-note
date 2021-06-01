use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use getopts::Matches;

use crate::symbol_table::SymbolTable;

pub mod parser;
pub mod code;
pub mod symbol_table;

pub struct Config {
    filename: String,
    output: Option<String>,
}

impl Config {
    pub fn new(matches: &Matches) -> Result<Config, &'static str> {
        if matches.free.len() < 1 {
            return Err("not enough arguments");
        }
        let filename = matches.free[0].clone();
        let output = matches.opt_str("o");

        Ok(Config { filename, output })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename.clone())?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let commands = parser::parse(&config.filename)?;

    let mut s_table = SymbolTable::new();
    s_table.map_symbols(&commands);
    if let Some(path) = config.output {
        code::compile_to_file(&commands, &s_table, path);
    } else {
        code::compile_to_stdout(&commands, &s_table);
    }

    Ok(())
}
