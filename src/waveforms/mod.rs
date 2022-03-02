pub mod oscillator;
pub mod oscillator_handle;
pub mod waveform;

// or 44_100?
pub const SAMPLE_RATE: u32 = 48_000;

pub const NOTE_A: f32 = 440.0;
pub const NOTE_B: f32 = 493.88;
pub const NOTE_C: f32 = 523.25;
pub const NOTE_D: f32 = 587.33;
pub const NOTE_E: f32 = 659.25;
pub const NOTE_F: f32 = 698.46;
pub const NOTE_G: f32 = 783.99;
pub const NOTE_A2: f32 = 880.00;
pub const NOTE_B2: f32 = 987.77;
pub const NOTE_C2: f32 = 1046.50;

// C1 -> B8 = 96 keys
