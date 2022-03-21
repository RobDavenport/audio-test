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
