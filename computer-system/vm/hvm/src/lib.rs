use config::Config;
use std::error::Error;

pub mod code_writer;
pub mod config;
pub mod parser;
pub mod symbol_table;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let command_list = parser::parse(&config.filename);
    if config.output.is_none() {
        code_writer::write_to_stdout(command_list.unwrap(), &config);
    } else {
        code_writer::write_to_file(command_list.unwrap(), &config);
    }
    Ok(())
}
