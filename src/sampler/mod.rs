use crate::instrument;
use crate::timing;
use instrument::instrument::Instrument;
use rodio::Source;
use timing::tempo::Tempo;

/// Number of playback channels.
const CHANNELS: u16 = 1;

/// Sample rate of playback.
const SAMPLE_RATE: u32 = 44_100;

pub struct Looper {
    sink: rodio::Sink,
}

impl From<rodio::Sink> for Looper {
    fn from(sink: rodio::Sink) -> Self {
        // wait for playing
        sink.pause();
        Self { sink }
    }
}

impl Looper {
    /// Plays a mixed pattern repeatedly for given amount of repeats
    pub fn add_repeated(&self, tempo: &Tempo, instruments: &[Instrument], nr_of_repeats: usize) {
        for _ in 0..=nr_of_repeats {
            self.add_once(tempo, instruments);
        }
    }

    pub fn play_till_end(&self) {
        // all samples are in place...play
        self.sink.play();
        // and wait
        self.sink.sleep_until_end();
    }

    pub fn add_once(&self, tempo: &Tempo, instruments: &[Instrument]) {
        let (controller, mixer) = rodio::dynamic_mixer::mixer(CHANNELS, SAMPLE_RATE);

        for instrument in instruments.iter() {
            // TODO, we could reuse the source...as it is already buffered here 3 lines below, and
            // keep it in the looper
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
