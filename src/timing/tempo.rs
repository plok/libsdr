/// Represents the playback tempo (beats per minute).
pub struct Tempo(pub u16);

impl From<u16> for Tempo {
    #[inline]
    fn from(v: u16) -> Tempo {
        Tempo(v)
    }
}
