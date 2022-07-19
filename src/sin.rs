use std::f32::consts::TAU;

use crate::{TARGET_SAMPLE_RATE};

const LUT_BIT_LENGTH: u32 = 16;
const LUT_ENTRY_COUNT: usize = 2_usize.pow(LUT_BIT_LENGTH);

static mut SIN_TABLE: &mut [f32; LUT_ENTRY_COUNT as usize] = &mut [0.0; LUT_ENTRY_COUNT];

// TODO: Make this work with 4th of SIN
pub fn init_sin_lut() {
    println!("SIN_TABLE Entry Count: {}", LUT_ENTRY_COUNT);
    unsafe {
        SIN_TABLE.iter_mut().enumerate().for_each(|(i, value)| {
            *value = f32::sin((i as f32 * TAU) / (LUT_ENTRY_COUNT as f32));
    })};
}

pub fn lookup(phase: u64) -> f32 {
    let index = (phase >> (u64::BITS - LUT_BIT_LENGTH)) as usize;
    unsafe { SIN_TABLE[index] }
}

fn get_w() -> f32 {
    u64::MAX as f32 / TARGET_SAMPLE_RATE as f32
}

pub fn get_delta_p(frequency: f32) -> u64 {
    (frequency * get_w()) as u64
}