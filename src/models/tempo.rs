/// Represents the playback tempo (beats per minute).
pub struct Tempo(pub u16);

impl From<u16> for Tempo {
    /// converts creates a new Tempo instance and stores the given beats per minute
    ///
    /// # Examples
    /// `120`
    fn from(beats_per_minute: u16) -> Tempo {
        Tempo(beats_per_minute)
    }
}
