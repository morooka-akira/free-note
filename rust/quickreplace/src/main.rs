use std::{env, fs};

use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(data) => data,
        Err(err) => {
            eprintln!(
                "{} failed to read from file '{} : {:?}'",
                "Eerror".red().bold(),
                args.filename,
                err
            );
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(err) => {
            eprintln!(
                "{} failed to replace text '{} : {:?}'",
                "Eerror".red().bold(),
                args.filename,
                err
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}
        Err(err) => {
            eprintln!(
                "{} failed to write to file '{} : {:?}'",
                "Eerror".red().bold(),
                args.output,
                err
            );
            std::process::exit(1);
        }
    }

    println!("{:?}", args);
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string in another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <filename> <output>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let re = regex::Regex::new(target)?;
    Ok(re.replace_all(text, replacement).to_string())
}
