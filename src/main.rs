mod pattern;
use rand_distr::{Binomial, Distribution };
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};
use audio_thread_priority;
fn main() {
    let mut var = pattern::Pattern::new(8);


        match audio_thread_priority::promote_current_thread_to_real_time(128, 48000) {
                Ok(_h) => {
                                println!("this thread is now bumped to real-time priority.") 
                                        }
                    Err(_e) => { println!("could not bump to real time.") }
                      }
    var.add_hit(3,  pattern::Hit{ length: 1, velocity: 123});
    var.add_hit(1,  pattern::Hit{ length: 2, velocity: 123});
    println!("Hello, world!");
    let bin = Binomial::new(128, 0.8).unwrap();

    // Decode that sound file into a source

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let (_sstream, sstream_handle) = OutputStream::try_default().unwrap();

    let file = BufReader::new(File::open("examples/kick.wav").unwrap());
        let source = Decoder::new(file).unwrap();
        let buffered = source.buffered();

    let sfile = BufReader::new(File::open("examples/snare.wav").unwrap());
        let ssource = Decoder::new(sfile).unwrap();
        let sbuffered = ssource.buffered();
    let mut i = 0.0;
    loop {
    
        i+=1.0;
        
        
        let k = buffered.clone().convert_samples();
        let s = sbuffered.clone().amplify(0.5).convert_samples();

        let _result = stream_handle.play_raw(k);
        if i % 8.0 == 0.0 {
            let _result = sstream_handle.play_raw(s);
        }
        if i==8.0 {i = 0.0};
std::thread::sleep(std::time::Duration::from_millis(80));

    }
}
