use crate::timing;
use crate::Tempo;
use rodio::Source;
use std::io::BufReader;

use crate::instrument;
use instrument::instrument::Instrument;

/// Number of playback channels.
/// Chaning this to two gives a better sound, but breaks the play_repeat method to only play the
/// first measurement
const CHANNELS: u16 = 1;

/// Sample rate of playback.
const SAMPLE_RATE: u32 = 44_100;

pub fn create_sample(
    tempo: &Tempo,
    instruments: Vec<Instrument>,
) -> Box<dyn Source<Item = i16> + Send> {
    let (controller, mixer) = rodio::dynamic_mixer::mixer(CHANNELS, SAMPLE_RATE);

    for instrument in instruments.iter() {
        let file = std::fs::File::open(&instrument.source_path).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file))
            .unwrap()
            .buffered();

        for (i, step) in instrument.pattern.iter().enumerate() {
            if !step {
                continue;
            }
            let delay = timing::step_duration(tempo) * (i as u32);
            controller.add(source.clone().amplify(instrument.amplify).delay(delay));
        }
    }

    Box::new(mixer)
}

/// Plays a mixed pattern repeatedly.
pub fn play_once(tempo: &Tempo, mix: Box<dyn Source<Item = i16> + Send>) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // play the pattern
    let _res = stream_handle.play_raw(mix.convert_samples());

    // sleep for the duration of a single measure
    std::thread::sleep(timing::measure_duration(tempo));
}

/// Plays a mixed pattern repeatedly.
pub fn play_repeat(tempo: &Tempo, mix: Box<dyn Source<Item = i16> + Send>) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    // play the pattern
    let _res = stream_handle.play_raw(
        mix
            // forward pad with trailing silence
            .delay(timing::delay_pad_duration(&tempo, 0))
            // trim to measure length
            .take_duration(timing::measure_duration(&tempo))
            .repeat_infinite()
            .convert_samples(),
    );
    // sleep forever
    std::thread::park();
}
