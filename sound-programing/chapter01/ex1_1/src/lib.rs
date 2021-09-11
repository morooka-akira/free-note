use getopts::Matches;

pub mod pcm;

#[derive(Debug, Clone)]
pub struct Config {
	pub filename: String,
}

impl Config {
	pub fn new(matches: &Matches) -> Result<Config, &'static str> {
		if matches.free.len() < 1 {
			return Err("not enough arguments");
		}
		let filename = matches.free[0].clone();

		Ok(Config { filename })
	}
}

pub fn run(config: Config) {
	println!("Hello lib");
	pcm::wave_read_16bit_mono(&config.filename);
}
