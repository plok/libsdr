use rodio::Source;
use std::fs::File;
use std::io::BufReader;

const SPEED: u64 = 100_000_000;
const KICK_SAMPLES: [u8; 8] = [1, 0, 0, 0, 1, 1, 0, 0];
const SNAR_SAMPLES: [u8; 8] = [0, 0, 1, 0, 0, 0, 1, 0];
const INTERVAL_ACCURARY: u32 = 1;

fn main() {
    std::thread::spawn(move || {
        play_beat("examples/kick.wav", &KICK_SAMPLES);
    });

    std::thread::spawn(move || {
        play_beat("examples/snare.wav", &SNAR_SAMPLES);
    });

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn play_beat(file_name: &str, samples: &[u8]) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let source_kick_buffered = rodio::Decoder::new(BufReader::new(File::open(file_name).unwrap()))
        .unwrap()
        .buffered()
        .convert_samples();
    // Not too sure about this one or the atomic timer
    let spin_sleeper = spin_sleep::SpinSleeper::new(INTERVAL_ACCURARY);
    let mut i = 0;
    loop {
        if samples[i] == 1 {
            let _res = stream_handle.play_raw(source_kick_buffered.clone());
        }
        i += 1;
        if i == samples.len() {
            i = 0;
        }
        spin_sleeper.sleep_ns(SPEED);
    }
}
