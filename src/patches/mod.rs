mod algorithm;
mod feedback;
mod operator;
mod patch;

pub use algorithm::*;
pub use feedback::*;
pub use operator::*;
pub use patch::*;

pub const OPERATOR_COUNT: usize = 4;
pub const AMPLIFICATION: f32 = 25.0;
pub const ENV_DB: f32 = 96.0;

pub(crate) fn attenuation(max_level: u8) -> f32 {
    let db = -(ENV_DB / (u8::MAX as f32 + 1.0)) * (u8::MAX - max_level) as f32;
    10f32.powf(db / 20.0)
}
