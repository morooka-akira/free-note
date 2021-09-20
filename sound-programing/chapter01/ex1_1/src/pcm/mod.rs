use std::fs::File;
use std::io::Read;

pub struct MonoPcm {
	pub fs: u32,     /* 標本化周波数: samples per sec */
	pub bits: u16,   /* 量子化精度: bits per sample */
	pub length: u32, /* データ長: data chunk size */
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
	let mut file = File::open(file_name).expect("file not found.");

	let riff: Vec<char> = read_4byte_to_vec(&mut file);
	println!("riff: {:?}", riff);

	let riff_chunk_size = read_4byte_to_u32(&mut file);
	println!("riff_chunk_size: {}", riff_chunk_size as u32);

	let file_format_type: Vec<char> = read_4byte_to_vec(&mut file);
	println!("file_format_type: {:?}", file_format_type);

	let fmt_chunk_id: Vec<char> = read_4byte_to_vec(&mut file);
	println!("fmt_chunk_ID: {:?}", fmt_chunk_id);

	let fmt_chunk_size = read_4byte_to_u32(&mut file);
	println!("fmt_chunk_size: {}", fmt_chunk_size as u32);

	let wave_format_type = read_2byte_to_u16(&mut file);
	println!("wave_format_type: {:?}", wave_format_type);

	let channel = read_2byte_to_u16(&mut file);
	println!("channel: {:?}", channel);

	let samples_per_sec = read_4byte_to_u32(&mut file);
	println!("samples_per_sec: {:?}", samples_per_sec);

	let bytes_per_sec = read_4byte_to_u32(&mut file);
	println!("bytes_per_sec: {:?}", bytes_per_sec);

	let block_size = read_2byte_to_u16(&mut file);
	println!("block_size: {:?}", block_size);

	let bits_per_sample = read_2byte_to_u16(&mut file);
	println!("bits_per_sample: {:?}", bits_per_sample);

	let data_chunk_id: Vec<char> = read_4byte_to_vec(&mut file);
	println!("data_chunk_id: {:?}", data_chunk_id);

	let data_chunk_size = read_4byte_to_u32(&mut file);
	println!("data_chunk_size: {:?}", data_chunk_size);

	let mut pcm = MonoPcm::new();
	pcm.fs = samples_per_sec;
	pcm.bits = bits_per_sample;
	pcm.length = data_chunk_size;

	let sample: f32 = 32768.0;
	for _ in 0..data_chunk_size {
		let data = read_2byte_to_u16(&mut file);
		let s = f32::from(data) / sample;
		pcm.s.push(s);
	}
	return pcm;
}

fn read_4byte_to_vec(file: &mut File) -> Vec<char> {
	let mut buf: [u8; 1] = [0; 1];
	let mut vec: Vec<char> = vec![];
	for _ in 0..4 {
		file.read(&mut buf).expect("not read byte.");
		vec.push(buf[0] as char);
	}
	return vec;
}

fn read_4byte_to_u32(file: &mut File) -> u32 {
	let mut buf: [u8; 4] = [0; 4];
	file.read(&mut buf).expect("not read byte.");
	let int = u32::from_le_bytes(buf);
	return int;
}

fn read_2byte_to_u16(file: &mut File) -> u16 {
	let mut buf: [u8; 2] = [0; 2];
	file.read(&mut buf).expect("not read byte.");
	let int = u16::from_le_bytes(buf);
	return int;
}
