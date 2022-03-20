use std::{f32::consts::TAU, sync::Arc};

use parking_lot::RwLock;

use super::oscillator::Waveform;

#[derive(Clone)]
pub struct PatchHandle {
    pub patch: Arc<RwLock<Patch>>,
}
impl PatchHandle {
    pub fn new(patch: Patch) -> Self {
        Self {
            patch: Arc::new(RwLock::new(patch)),
        }
    }

    pub fn get_active(&self) -> bool {
        self.patch.read().active
    }

    pub fn set_frequency(&self, frequency: f32) {
        self.patch.write().base_frequency = frequency
    }

    pub fn set_waveform(&self, operator_index: usize, waveform: Waveform) {
        self.patch.write().operators.operators[operator_index].waveform = waveform;
    }

    pub fn set_active(&self, active: bool) {
        self.patch.write().active = active;
    }

    pub fn write_to_buffer(&mut self, data: &mut [f32], channels: u16) {
        let mut lock = self.patch.write();
        lock.write_to_buffer(data, channels)
    }
}

pub enum FrequencyMultiplier {
    Eigth,
    Quarter,
    Half,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl Default for FrequencyMultiplier {
    fn default() -> Self {
        Self::One
    }
}

#[derive(Default)]
pub struct Operator {
    waveform: Waveform,
    max_level: u8,
    frequency_multiplier: FrequencyMultiplier,
    detune: i8,
}

pub enum Algorithm {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Default for Algorithm {
    fn default() -> Self {
        Self::One
    }
}

#[derive(Default)]
pub struct Operators {
    operators: [Operator; 4],
}

pub struct Patch {
    pub(crate) active: bool,
    sample_rate: u32,
    pub(crate) clock: u32,
    pub(crate) base_frequency: f32,
    operators: Operators,
    algorithm: Algorithm,
}

impl Patch {
    pub fn new(base_frequency: f32, sample_rate: u32) -> Self {
        Self {
            active: false,
            sample_rate,
            clock: 0,
            base_frequency,
            operators: Operators::default(),
            algorithm: Algorithm::default(),
        }
    }

    fn func(&self, tone: f32) -> f32 {
        todo!()
    }
}

impl Iterator for Patch {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.active {
            self.clock += 1;

            let tone = self.clock as f32 * self.base_frequency * TAU / self.sample_rate as f32;

            Some(self.func(tone))
        } else {
            None
        }
    }
}
