use crate::models::instrument::Instrument;

pub struct Track<'a> {
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
    /// Each note can have a value between 0 and 128.
    /// 128 is hit at 100%
    /// 0 is no hit
    /// 64 is hit at 50%
    ///
    /// Annotated without the dashes and each hit at a 100%
    /// `128 0 0 0 128 0 128 0 128 0 0 0 128 0 128 0`
    pub hits: Vec<u8>,

    /// The instrument that belongs to this track
    pub instrument: &'a Instrument,
}
