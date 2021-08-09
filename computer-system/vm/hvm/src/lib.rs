use config::Config;
use std::error::Error;
use std::path::Path;

pub mod code_writer;
pub mod config;
pub mod parser;
pub mod symbol_table;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = Path::new(&config.filename);
    if file.is_dir() {
        let dirs = file.read_dir().expect("read_dir call failed");
        let files: Vec<String> = dirs
            .filter_map(|e| e.ok())
            .filter(|dir| dir.path().extension().unwrap() == "vm")
            .filter_map(|p| Some(p.path().to_str()?.into()))
            .collect();
        let mut commands = Vec::new();
        for file in files {
            let command_list = parser::parse(&file);
            commands.append(&mut command_list.unwrap());
        }
        if config.output.is_none() {
            code_writer::write_to_stdout(commands, &config);
        } else {
            code_writer::write_to_file(commands, &config);
        }
    } else {
        let command_list = parser::parse(&config.filename);
        if config.output.is_none() {
            code_writer::write_to_stdout(command_list.unwrap(), &config);
        } else {
            code_writer::write_to_file(command_list.unwrap(), &config);
        }
    }
    Ok(())
}
