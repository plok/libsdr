use crate::models;
use crate::sampler::{CHANNELS, SAMPLE_RATE};
use crate::timing;
use models::pattern::Pattern;
use rodio::{dynamic_mixer::mixer, source::Buffered, Decoder, Sink, Source};
use std::{collections::HashMap, fs::File, io::BufReader};
use timing::{measure_duration, step_duration};

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
    pub fn add_repeated(&mut self, pattern: &Pattern, nr_of_repeats: usize) {
        // for the number of repeats given
        for _ in 0..=nr_of_repeats {
            // append a new sample to the sink
            self.add_once(pattern);
        }
    }

    /// Appends a sample only once to the sink, prepares the mix for all the given instruments
    pub fn add_once(&mut self, pattern: &Pattern) {
        // initialize a mixer and a controller instance for the given amount of channels and the
        // sample rate
        let (controller, mixer) = mixer(CHANNELS, SAMPLE_RATE);

        for track in pattern.tracks.iter() {
            // See if we already have a buffered file decoder ready to re-used, otherwise
            // initialize and add to our hashmap
            let source = self
                .source_buffers
                .entry(track.instrument.source_path.clone())
                .or_insert({
                    let file = File::open(&track.instrument.source_path).unwrap();
                    let file_buffer = BufReader::new(file);
                    Decoder::new(file_buffer).unwrap().buffered()
                });

            // For each step that is marked at a hit, add a step to the controller
            for (i, step) in track.hits.iter().enumerate() {
                if step < &0 {
                    continue;
                }

                let amplify = (step.clone() as f32) / 128.0;
                let delay = step_duration(pattern) * (i as u32);
                controller.add(source.clone().amplify(amplify).delay(delay));
            }
        }

        // append the mix for all instruments to the sink
        self.sink
            .append(mixer.take_duration(measure_duration(pattern)));
    }

    /// Plays the samples till the end
    pub fn play_till_end(&self) {
        // all samples are in place...play
        self.sink.play();
        // and wait
        self.sink.sleep_until_end();
    }
}
