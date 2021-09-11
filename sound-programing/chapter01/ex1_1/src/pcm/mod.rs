use std::fs::File;
use std::io::Read;

pub struct MonoPcm {
	pub fs: i32,     /* 標本化周波数: samples per sec */
	pub bits: i32,   /* 量子化精度: bits per sample */
	pub length: i32, /* データ長: data chunk size */
	pub s: Vec<f32>, /* 音声データ */
}

impl MonoPcm {
	fn new() -> MonoPcm {
		MonoPcm {
			fs: 0,
			bits: 0,
			length: 0,
			s: Vec::new(),
		}
	}
}

pub fn wave_read_16bit_mono(file_name: &str) -> MonoPcm {
	println!("wave_read_16bit_mono {}", file_name);
	let pcm = MonoPcm::new();
	pcm
}
