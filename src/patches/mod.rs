mod algorithm;
mod envelope;
mod feedback;
mod operator;
mod patch;

pub use algorithm::*;
pub use envelope::*;
pub use feedback::*;
pub use operator::*;
pub use patch::*;

pub const OPERATOR_COUNT: usize = 4;
pub const AMPLIFICATION: f32 = 25.0;
pub const ENV_DB: f32 = 96.0;
pub const ATTENUATION_MAX: u16 = 2u16.pow(10);

fn calculate_attenuation(amount: usize, max: usize) -> f32 {
    let db = -(ENV_DB / (max as f32 + 1.0)) * amount as f32;
    10f32.powf(db / 20.0)
}

pub(crate) fn init_attenuation_table() {
    unsafe {
        ATTENUATION_TABLE_10
            .iter_mut()
            .enumerate()
            .for_each(|(index, output)| {
                *output = calculate_attenuation(index, ATTENUATION_MAX as usize)
            });

        ATTENUATION_TABLE_8
            .iter_mut()
            .enumerate()
            .for_each(|(index, output)| *output = calculate_attenuation(index, u8::MAX as usize))
    }
}

pub(crate) fn attenuation_table_u10(index: u16) -> f32 {
    unsafe { ATTENUATION_TABLE_10[index as usize] }
}

pub(crate) fn attenuation_table_u8(index: u8) -> f32 {
    unsafe { ATTENUATION_TABLE_8[index as usize] }
}

static mut ATTENUATION_TABLE_10: &mut [f32; ATTENUATION_MAX as usize + 1] =
    &mut [0.0; ATTENUATION_MAX as usize + 1];

static mut ATTENUATION_TABLE_8: &mut [f32; u8::MAX as usize + 1] = &mut [0.0; u8::MAX as usize + 1];
