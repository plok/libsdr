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

    /// The name of the instrument.
    ///
    /// #Example
    /// `Kick drum`
    pub name: String,

    /// The type of instrument this is,
    ///
    /// # Example
    /// `InstrumentType::Snare`
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
        })
    }
}
