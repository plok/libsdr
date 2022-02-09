use crate::instrument;
use crate::timing;
use instrument::instrument::Instrument;
use rodio::Source;
use timing::tempo::Tempo;
/// Number of playback channels.
/// first measurement
const CHANNELS: u16 = 1;

/// Sample rate of playback.
const SAMPLE_RATE: u32 = 44_100;

pub struct Looper {
    sink: rodio::Sink,
}

impl From<rodio::Sink> for Looper {
    fn from(sink: rodio::Sink) -> Self {
        Self { sink }
    }
}

impl Looper {
    /// Plays a mixed pattern repeatedly for given amount of repeats
    pub fn play_repeat(&self, tempo: &Tempo, instruments: &[Instrument], nr_of_repeats: usize) {
        for _ in 0..=nr_of_repeats {
            self.play_once(tempo, instruments);
        }
    }

    pub fn play_till_end(&self) {
        self.sink.sleep_until_end();
    }

    pub fn play_once(&self, tempo: &Tempo, instruments: &[Instrument]) {
        let (controller, mixer) = rodio::dynamic_mixer::mixer(CHANNELS, SAMPLE_RATE);

        for instrument in instruments.iter() {
            let file = std::fs::File::open(&instrument.source_path).unwrap();
            let file_buffer = std::io::BufReader::new(file);
            let source = rodio::Decoder::new(file_buffer).unwrap().buffered();

            for (i, step) in instrument.pattern.iter().enumerate() {
                if !step {
                    continue;
                }
                let delay = timing::step_duration(tempo) * (i as u32);
                controller.add(source.clone().amplify(instrument.amplify).delay(delay));
            }
        }
        self.sink
            .append(mixer.take_duration(timing::measure_duration(tempo)));
    }
}
