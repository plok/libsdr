use crate::instrument;
use crate::sampler::{CHANNELS, SAMPLE_RATE};
use crate::timing;
use instrument::instrument::Instrument;
use rodio::{dynamic_mixer::mixer, source::Buffered, Decoder, Sink, Source};
use std::{collections::HashMap, fs::File, io::BufReader};
use timing::{measure_duration, step_duration, tempo::Tempo};

/// Defines a sampler which is able to play a sequence of instrument beats in a mix
pub struct Sampler {
    /// The sink to append the samples to
    sink: Sink,

    /// A buffered hashmap containing previously loaded file decoder buffers
    source_buffers: HashMap<String, Buffered<Decoder<BufReader<File>>>>,
}

impl From<Sink> for Sampler {
    /// Converts a sink into a Sampler instance.
    fn from(sink: Sink) -> Self {
        // wait for playing
        sink.pause();
        Self {
            sink,
            source_buffers: HashMap::new(),
        }
    }
}

impl Sampler {
    /// Plays a mixed pattern repeatedly for given amount of repeats
    pub fn add_repeated(
        &mut self,
        tempo: &Tempo,
        instruments: &[Instrument],
        nr_of_repeats: usize,
    ) {
        for _ in 0..=nr_of_repeats {
            self.add_once(tempo, instruments);
        }
    }

    /// Plays a sample only once, prepares the mix for all the given instruments
    pub fn add_once(&mut self, tempo: &Tempo, instruments: &[Instrument]) {
        let (controller, mixer) = mixer(CHANNELS, SAMPLE_RATE);

        for instrument in instruments.iter() {
            // See if we already have a buffered file decoder ready to re-used, otherwise
            // initialize and add to our hashmap
            let source = self
                .source_buffers
                .entry(instrument.source_path.clone())
                .or_insert({
                    let file = File::open(&instrument.source_path).unwrap();
                    let file_buffer = BufReader::new(file);
                    Decoder::new(file_buffer).unwrap().buffered()
                });
            for (i, step) in instrument.pattern.iter().enumerate() {
                if !step {
                    continue;
                }
                let delay = step_duration(tempo) * (i as u32);
                controller.add(source.clone().amplify(instrument.amplify).delay(delay));
            }
        }
        self.sink
            .append(mixer.take_duration(measure_duration(tempo)));
    }

    /// Plays the samples till the end
    pub fn play_till_end(&self) {
        // all samples are in place...play
        self.sink.play();
        // and wait
        self.sink.sleep_until_end();
    }
}