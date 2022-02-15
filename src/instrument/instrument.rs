use std::{io::Error, io::ErrorKind, path::Path};

/// Defines a interstrument for a single beat pattern.
pub struct Instrument {
    /// The file path of which correlates to this instrument
    ///
    /// # Examples
    /// `examples/kick.wav`
    /// `examples/snare.wav`
    /// `examples/crash1.wav`
    pub source_path: String,

    /// The pattern which indicates on which notes to trigger this instrument within the beat
    /// For example, if you have a beat that contains 4 measures, and 4 notes per measure, we can
    /// end up having 16 notes over 4 measures.
    ///
    /// # Examples
    ///
    /// Take the following beat sequence into consideration
    /// |         |         |         |
    /// | 1 0 0 0 | 1 0 1 0 | 1 0 0 0 | 1 0 1 0
    ///
    /// but annotated without the dashes
    /// `1 0 0 0 1 0 1 0 1 0 0 0 1 0 1 0`
    pub pattern: Vec<bool>,

    /// Defines the amplification of the sound to play when triggered
    /// Ranges between 0.0 and 1.0 and defaults to 1.0
    ///
    /// # Examples
    /// `0.5` (plays at 50% volume)
    /// `1.0` (plays at 100% volume)
    /// `0.0` (what's that?)
    pub amplify: f32,

    /// The name of the instrument.
    ///
    /// #Example
    /// `Kick drum`
    pub name: String,

    /// The type of instrument this is,
    ///
    /// # Example
    /// `Snare`
    pub instrument_type: InstrumentType,
}

pub enum InstrumentType {
    Snare,
    Kick,
    Tom,
    Cymbal,
}

impl Instrument {
    /// Will attempt to create a new instrument instance.
    ///
    /// If the given source_path is not found, this method will throw an exception.
    /// Other wise, a new instance will be returned succesfully
    pub fn try_new(
        name: &str,
        instrument_type: InstrumentType,
        source_path: &str,
        pattern: Vec<u8>,
        amplify: Option<f32>,
    ) -> Result<Self, Error> {
        // Validate that the file we want to load, it actually present on the hard drive
        if !Path::new(source_path).exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("{} is not found on the file system", source_path),
            ));
        }

        // Success, return the result
        Ok(Instrument {
            name: name.to_string(),
            instrument_type,
            source_path: source_path.to_string(),
            pattern: pattern.iter().map(|p| p != &0).collect(),
            amplify: match amplify {
                Some(val) => val,
                _ => 1.0,
            },
        })
    }
}
