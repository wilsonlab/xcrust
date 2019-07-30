use chrono::{DateTime, Duration, Utc};
pub mod mwl_ad;
pub mod ascii_draw;

/// A raw `Spike` is a set of waveforems and a timestamp
/// The voltage and time parameters are both abstract,
/// and can only be interpreted in a larger context.
///
/// The context for interpreting waveforms should be in
/// the container for these `Spike`s - for instance, an AD
/// spike file will provide the sampling rate and a function
/// for converting a u8 into absolute voltage
#[derive(Debug)]
pub struct Spike<V, T> {
    pub waveforms: Vec<Vec<V>>,
    pub time: T,
}

/// A `SpikeSI` (Spike with SI units) provides some context
/// to a spike: its waveforms are Voltages (in V) at the tips
/// of the electrode (amplifier gain has been divided out)
/// Its timestamp is the absolute UTC time of the triggering
/// sample. If triggering is due to a specific channel, than
/// channel is at index `triggering_channel`.
pub struct SpikeSI {
    pub si_spike: Spike<f64, DateTime<Utc>>,
    pub si_sampling_period: Duration,
    pub si_triggering_sample: u32,
    pub si_triggering_channel: Option<u32>,
}
