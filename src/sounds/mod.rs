use cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::slice::ToFrameSliceMut;
use dasp::{signal, Sample, Signal};

pub struct SoundBank {
    sounds: Vec<Sound>,
}

impl SoundBank {
    pub fn new() -> Self {
        return SoundBank { sounds: Vec::new() };
    }

    pub fn load_sound(&mut self, file: &str) {
        let assets = find_folder::Search::ParentsThenKids(5, 5)
            .for_folder("examples")
            .unwrap();

        let reader = hound::WavReader::open(assets.join(file)).unwrap();

        let spec = reader.spec();

        println!("Spec: {:?}", spec);
        let samples: Vec<i16> = reader.into_samples().map(|s| s.unwrap()).collect();

        let sound = Sound::new(file, samples);
        self.sounds.push(sound);
    }
}

pub struct Sound {
    name: String,
    samples: Vec<i16>,
    playing: bool,
}

impl Sound {
    fn new(name: &str, samples: Vec<i16>) -> Sound {
        return Sound {
            name: name.to_string(),
            samples,
            playing: false,
        };
    }
}

pub struct Player {
    sound_bank: SoundBank,
}

pub fn play(sound_bank: SoundBank) -> Result<(), anyhow::Error> {
    let mut main_signal: signal::Equilibrium<[i16; 2]> = signal::equilibrium();
    let mut vecs = Vec::new();
    for sound in sound_bank.sounds {
       let  signal =  signal::from_interleaved_samples_iter::<_, [i16; 2]>(sound.samples);
       vecs.push(signal);
    }



    
    let mut frames = main_signal.until_exhausted();
    // Initialise CPAL.
    let host = cpal::default_host();
    let device: cpal::Device = host
        .default_output_device()
        .expect("failed to find a default output device");

    let config = cpal::StreamConfig {
        channels: 2,
        sample_rate: cpal::SampleRate(44100),
        buffer_size: cpal::BufferSize::Default,
    };

    // A channel for indicating when playback has completed.
    let (complete_tx, complete_rx) = std::sync::mpsc::sync_channel(1);

    // Create and run the CPAL stream.
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let data_fn = move |data: &mut [i16], _info: &cpal::OutputCallbackInfo| {
        let buffer: &mut [[i16; 2]] = data.to_frame_slice_mut().unwrap();
        for out_frame in buffer {
            match frames.next() {
                Some(frame) => {
                    *out_frame = frame;
                }
                None => {
                    complete_tx.try_send(()).ok();
                    *out_frame = dasp::Frame::EQUILIBRIUM;
                }
            }
        }
    };
    let stream = device.build_output_stream(&config, data_fn, err_fn)?;
    stream.play().unwrap();

    // Block until playback completes.
    complete_rx.recv().unwrap();
    stream.pause().ok();
    Ok(())

}
