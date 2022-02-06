extern crate libsdr;

use libsdr::timing::tempo::Tempo;

use libsdr::instrument::instrument::Instrument;

use libsdr::sampler;

fn main() {
    let tempo = Tempo::from(120);

    let instruments = vec![
        Instrument::new(
            "assets/kick.wav",
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1].to_vec(),
            //          |           |           |          |
            None,
        ),
        Instrument::new(
            "assets/snare.wav",
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1].to_vec(),
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

    let sample = sampler::create_sample(&tempo, &instruments);
    //       sampler::play_once(&tempo, sample);
    sampler::play_repeat(&tempo, sample);
}
