use super::track::Track;

use super::tempo::Tempo;

pub struct Pattern<'a> {
    /// Defines the MeasureType for this pattern
    ///
    /// # Examples
    /// `4/4`
    /// `3/4`
    /// `6/8`
    pub measure_type: (u8, u8),

    /// The number of measures for this pattern
    pub nr_of_measures: u8,

    /// Defines the amount of beats per minute.
    ///
    /// # Example
    /// `120`
    pub tempo: &'a Tempo,

    /// defines the tracks included in this beat.
    pub tracks: Vec<Track<'a>>,
}

impl<'a> Pattern<'_> {
    /// Initializes a new Pattern instance.
    pub fn new(
        measure_type: (u8, u8),
        nr_of_measures: u8,
        tempo: &'a Tempo,
        tracks: Vec<Track<'a>>,
    ) -> Pattern<'a> {
        Pattern {
            measure_type,
            nr_of_measures,
            tempo,
            tracks,
        }
    }
}
