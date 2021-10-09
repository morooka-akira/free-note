use std::f32::consts::PI;

use getopts::Matches;
use pcm::MonoPcm;

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

pub fn run() {
    let mut pcm = MonoPcm::new();
    // 標本化周波数
    pcm.fs = 44100;
    // 量子化精度
    pcm.bits = 16;
    // 音の長さ
    pcm.length = pcm.fs * 2;
    // サイン波
    // // 振幅(音の大きさ)
    // let a = 0.1;
    // // 周波数音の高さ
    // let f0 = 500.0;
    // for n in 0..pcm.length {
    //     let a = a * (2.0 * PI * f0 * (n as f32) / (pcm.fs as f32)).sin();
    //     pcm.s.push(a);
    // }
    let duration = pcm.fs / 4;
    sine_wave(&mut pcm, 261.63, 0.1, duration);
    sine_wave(&mut pcm, 293.66, 0.1, duration);
    sine_wave(&mut pcm, 329.63, 0.1, duration);
    sine_wave(&mut pcm, 349.23, 0.1, duration);
    sine_wave(&mut pcm, 392.00, 0.1, duration);
    sine_wave(&mut pcm, 440.00, 0.1, duration);
    sine_wave(&mut pcm, 493.88, 0.1, duration);
    sine_wave(&mut pcm, 523.25, 0.1, duration);
    pcm::wave_write_16bit_mono(&pcm, "./abc.wav")
}

pub fn sine_wave(pcm: &mut MonoPcm, f0: f32, a: f32, duration: u32) {
    // サイン波
    for n in 0..duration {
        let d = a * (2.0 * PI * f0 * (n as f32) / (pcm.fs as f32)).sin();
        pcm.s.push(d);
    }
    // フェード処理
    for n in 0..(pcm.fs / 100) {
        pcm.s[n as usize] *= n as f32 / (0.01 * pcm.fs as f32);
        let i: u32 = duration - n - 1;
        pcm.s[i as usize] *= n as f32 / (0.01 * pcm.fs as f32);
    }
}
