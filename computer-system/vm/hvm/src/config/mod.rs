use getopts::Matches;

pub struct Config {
    pub filename: String,
    pub output: Option<String>,
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
