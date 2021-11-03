use std::fs::File;
use std::io::Read;
use std::io::Write;

pub struct MonoPcm {
    pub fs: u32,     /* 標本化周波数: samples per sec */
    pub bits: u16,   /* 量子化精度: bits per sample */
    pub length: u32, /* データ長: data chunk size */
    pub s: Vec<f32>, /* 音声データ */
}

impl MonoPcm {
    pub fn new() -> MonoPcm {
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
    pcm.length = data_chunk_size / 2;

    for _ in 0..pcm.length {
        let data = read_2byte_to_i16(&mut file);
        /* 音データを-1以上1未満の範囲に正規化する */
        let s: f32 = f32::from(data);
        let s: f32 = s / 32768.0;
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

fn read_2byte_to_i16(file: &mut File) -> i16 {
    let mut buf: [u8; 2] = [0; 2];
    file.read(&mut buf).expect("not read byte.");
    let int = i16::from_le_bytes(buf);
    return int;
}

// 書き込みがうまく行っていない
pub fn wave_write_16bit_mono(pcm: &MonoPcm, file_name: &str) {
    let mut file = File::create(file_name).expect("file not found.");
    let riff_chunk_id: [u8; 4] = [b'R', b'I', b'F', b'F'];
    let riff_chunk_size = 36 + pcm.length * 2;
    let file_format_type: [u8; 4] = [b'W', b'A', b'V', b'E'];
    let fmt_chunk_id: [u8; 4] = [b'f', b'm', b't', b' '];
    let fmt_chunk_size: u32 = 16;
    let wave_format_type: u16 = 1;
    let channel: u16 = 1;
    let samples_per_sec = pcm.fs;
    let bytes_per_sec = pcm.fs * u32::from(pcm.bits) / 8;
    let block_size: u16 = pcm.bits / 8;
    let bits_per_sample: u16 = pcm.bits;
    let data_chunk_id: [u8; 4] = [b'd', b'a', b't', b'a'];
    let data_chunk_size = pcm.length * 2;

    file.write(&riff_chunk_id).unwrap();
    file.write(&riff_chunk_size.to_le_bytes()).unwrap();
    file.write(&file_format_type).unwrap();
    file.write(&fmt_chunk_id).unwrap();
    file.write(&fmt_chunk_size.to_le_bytes()).unwrap();
    file.write(&wave_format_type.to_le_bytes()).unwrap();
    file.write(&channel.to_le_bytes()).unwrap();
    file.write(&samples_per_sec.to_le_bytes()).unwrap();
    file.write(&bytes_per_sec.to_le_bytes()).unwrap();
    file.write(&block_size.to_le_bytes()).unwrap();
    file.write(&bits_per_sample.to_le_bytes()).unwrap();
    file.write(&data_chunk_id).unwrap();
    file.write(&data_chunk_size.to_le_bytes()).unwrap();

    for n in 0..pcm.length {
        let mut data: f32 = (pcm.s[n as usize] + 1.0) / 2.0 * 65536.0;
        if data > 65535.0 {
            data = 65535.0
        } else if data < 0.0 {
            data = 0.0
        }
        let d: i32 = (data + 0.5) as i32 - 32768;
        let d: i16 = d as i16;
        file.write(&d.to_le_bytes()).unwrap();
    }
}
