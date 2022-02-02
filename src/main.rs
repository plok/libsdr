mod pattern;
mod sounds;

use audio_thread_priority;
use cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::slice::ToFrameSliceMut;
use dasp::{signal, Sample, Signal};

fn main() -> Result<(), anyhow::Error> {
    match audio_thread_priority::promote_current_thread_to_real_time(128, 44100) {
        Ok(_h) => {
            println!("this thread is now bumped to real-time priority.")
        }
        Err(_e) => {
            println!("could not bump to real time.")
        }
    }

    println!("Hello, world!");
    let mut bank =  sounds::SoundBank::new();
    bank.load_sound("output.wav");

    let _result = sounds::play(bank);
    // Read the interleaved samples and convert them to a signal.
    //let samples = reader.into_samples::<i32>().filter_map(Result::ok);

    //println!("Number of samples: {:?}",samples.len());
    //let samples = reader.samples().map(|s| s.unwrap());

    //let mut frames = signal.until_exhausted();
    // Initialise CPAL.
    //let host = cpal::default_host();
    //let device : cpal::Device = host
     //   .default_output_device()
      //  .expect("failed to find a default output device");

//    match device.name() {
//        Ok(v) => println!("Got device {:?} {:?}", v, spec.sample_rate),
//        Err(e) => println!("{:?}", e),
//    }

 //   if let Ok(conf) = device.default_output_config() {
 //       println!("    Default output stream config:\n      {:?}", conf);
 //   }
 //   let output_configs = match device.supported_output_configs() {
 //       Ok(f) => f.collect(),
 //       Err(e) => {
 //           println!("    Error getting supported output configs: {:?}", e);
 //           Vec::new()
  //      }
  //  };
  //  if !output_configs.is_empty() {
  //      println!("    All supported output stream configs:");
  //      for (config_index, config) in output_configs.into_iter().enumerate() {
  //          println!("     .{}. {:?}", config_index + 1, config);
  //      }
  //  }
    //println!("{:?}", device);

    // Create a stream config to match the wave format.
    //
    // NOTE: It's possible that the platform will not support the sample format, sample rate or
    // channel layout of the WAV file. In these cases, you may need to convert the data read from
    // the WAV file to a format compatible with one of the platform's supported stream
    // configurations.
        Ok(())

    // Decode that sound file into a source
}
