use std::time::Duration;
pub mod tempo;
use tempo::Tempo;

/// The number of steps in a measure.
///
/// When we use 4 notes per measure, and we have 4 measures, we should have 16 steps per measure
const STEPS_PER_MEASURE: usize = 16;

/// The number of beats in a measure.
const BEATS_PER_MEASURE: usize = 4;

/// Computes the duration of a measure.
///
/// # Literature
///
/// https://en.wikipedia.org/wiki/Pulses_per_quarter_note
/// This resolution is a measure of time relative to tempo since the tempo defines the length of a
/// quarter note and so the duration of each pulse. The resulting PPQN per MIDI-Clock is thus related
/// to the TimeBase in Microseconds defined as 60.000.000 x MicroTempo = Beats per minute.[1]
pub fn measure_duration(tempo: &Tempo) -> Duration {
    Duration::from_secs_f32(1.0 / (tempo.0 as f32 / 60.0 / BEATS_PER_MEASURE as f32))
}

/// Computes the duration of a step.
pub fn step_duration(tempo: &Tempo) -> Duration {
    measure_duration(tempo) / STEPS_PER_MEASURE as u32
}

/// Computes the duration to delay a mix with trailing silence when played on repeat.
/// This is necessary so that playback of the next iteration begins at the end
/// of the current iteration's measure instead of after its final non-silent step.
pub fn delay_pad_duration(tempo: &Tempo, trailing_silent_steps: usize) -> Duration {
    step_duration(tempo).mul_f32(delay_factor(tempo)) * trailing_silent_steps as u32
}

/// Computes a factor necessary for delay-padding a mix played on repeat.
pub fn delay_factor(tempo: &Tempo) -> f32 {
    -1.0 / 120.0 * tempo.0 as f32 + 2.0
}
