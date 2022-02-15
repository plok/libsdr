extern crate libsdr;

use libsdr::models::{
    instrument::Instrument, instrument::InstrumentType, tempo::Tempo, track::Track,
};
use libsdr::sampler::sampler;

fn main() {
    let tempo = Tempo::from(140);

    let kick = Instrument::try_new("Kick drum", InstrumentType::Kick, "assets/kick.wav").unwrap();
    let snare = Instrument::try_new("Snare", InstrumentType::Snare, "assets/snare.wav").unwrap();
    let ride =
        Instrument::try_new("Ride", InstrumentType::Cymbal, "assets/Ride_A/Ride_A_2.wav").unwrap();

    let beat1 = vec![
        Track {
            instrument: &kick,
            hits: [
                128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
            ]
            .to_vec(),
        },
        Track {
            instrument: &snare,
            hits: [0, 0, 50, 0, 0, 0, 50, 0, 0, 0, 50, 0, 0, 0, 50, 0].to_vec(),
        },
        Track {
            instrument: &ride,
            hits: [
                100, 0, 100, 0, 100, 0, 100, 0, 100, 0, 100, 0, 100, 0, 100, 0,
            ]
            .to_vec(),
        },
    ];

    let beat2 = vec![
        Track {
            instrument: &kick,
            hits: [0, 0, 128, 128, 0, 0, 128, 128, 128, 128, 0, 0, 0, 0, 0, 0].to_vec(),
        },
        Track {
            instrument: &snare,
            hits: [50, 50, 0, 0, 50, 50, 0, 0, 0, 0, 10, 20, 30, 40, 50, 50].to_vec(),
        },
    ];

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let mut looper: sampler::Sampler = sink.into();

    // Prepare samples
    for _ in 0..=4 {
        looper.add_repeated(&tempo, &beat1, 3);
        looper.add_once(&tempo, &beat2); // could also be add_repeated with 1
    }
    // play
    looper.play_till_end();
}
