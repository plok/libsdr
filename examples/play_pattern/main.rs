extern crate libsdr;

use libsdr::instrument::instrument::Instrument;
use libsdr::sampler;
use libsdr::timing::tempo::Tempo;

fn main() {
    let tempo = Tempo::from(140);

    let beat1 = vec![
        Instrument::new(
            "assets/kick.wav",
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1].to_vec(),
            //          |           |           |          |
            None,
        ),
        Instrument::new(
            "assets/snare.wav",
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0].to_vec(),
            //          |           |           |          |
            Some(0.4),
        ),
        Instrument::new(
            "assets/Ride_A/Ride_A_2.wav",
            [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0].to_vec(),
            //          |           |           |          |
            Some(0.8),
        ),
    ];

    let beat2 = vec![
        Instrument::new(
            "assets/kick.wav",
            [1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0].to_vec(),
            //          |           |           |          |
            None,
        ),
        Instrument::new(
            "assets/snare.wav",
            [0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1].to_vec(),
            //          |           |           |          |
            Some(0.4),
        ),
        Instrument::new(
            "assets/Ride_A/Ride_A_2.wav",
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec(),
            //          |           |           |          |
            Some(0.8),
        ),
    ];

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let mut looper: sampler::Looper = sink.into();

    // Prepare samples
    for _ in 0..=4 {
        looper.add_repeated(&tempo, &beat1, 3);
        looper.add_once(&tempo, &beat2); // could also be add_repeated with 1
    }
    // play
    looper.play_till_end();
}
