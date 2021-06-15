use config::Config;
use std::error::Error;

pub mod code_writer;
pub mod config;
pub mod parser;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
