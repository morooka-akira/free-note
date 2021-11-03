use std::f32::consts::PI;

use pcm::{wave_write_16bit_mono, MonoPcm};

pub mod pcm;

pub fn run() {
    write_nokogiri();
    write_rectangle()
}

// ノコギリ波から偶数の倍音を取り除いたものは矩形波になる
fn write_rectangle() {
    let mut pcm = MonoPcm::new();
    // 標本化周波数
    pcm.fs = 44100;
    // 量子化精度
    pcm.bits = 16;
    // 音の長さ
    pcm.length = pcm.fs * 1;
    pcm.s.resize(pcm.length as usize, 0.0);
    // // 周波数音の高さ
    let f0 = 500.0;
    let iter = (1..=44).filter(|n| n % 2 == 1);
    for i in iter {
        for n in 0..pcm.length {
            let a = 1.0 / i as f32 * (2.0 * PI * i as f32 * f0 * n as f32 / pcm.fs as f32).sin();
            pcm.s[n as usize] += a;
        }
    }
    let gain: f32 = 0.1;
    for n in 0..pcm.length {
        let a = pcm.s[n as usize] * gain;
        pcm.s[n as usize] = a;
    }
    wave_write_16bit_mono(&pcm, "kukei.wav")
}

fn write_nokogiri() {
    let mut pcm = MonoPcm::new();
    // 標本化周波数
    pcm.fs = 44100;
    // 量子化精度
    pcm.bits = 16;
    // 音の長さ
    pcm.length = pcm.fs * 1;
    pcm.s.resize(pcm.length as usize, 0.0);
    // // 周波数音の高さ
    let f0 = 500.0;
    for i in 1..=44 {
        for n in 0..pcm.length {
            let a = 1.0 / i as f32 * (2.0 * PI * i as f32 * f0 * n as f32 / pcm.fs as f32).sin();
            pcm.s[n as usize] += a;
        }
    }
    let gain: f32 = 0.1;
    for n in 0..pcm.length {
        let a = pcm.s[n as usize] * gain;
        pcm.s[n as usize] = a;
    }
    wave_write_16bit_mono(&pcm, "nokogiri.wav")
}
