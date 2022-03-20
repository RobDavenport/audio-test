use std::{f32::consts::TAU, sync::Arc};

use parking_lot::RwLock;

use super::oscillator::Waveform;

const AMPLIFICATION: f32 = 25.0;
const ENV_DB: f32 = 96.0;

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
        self.patch.write().operators[operator_index].waveform = waveform;
    }

    pub fn set_active(&self, active: bool) {
        self.patch.write().active = active;
    }

    pub fn write_to_buffer(&mut self, data: &mut [f32], channels: u16) {
        let mut lock = self.patch.write();
        lock.write_to_buffer(data, channels)
    }
}

#[derive(Clone)]
pub enum FrequencyMultiplier {
    OneEigth,
    OneFourth,
    OneHalf,
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

impl FrequencyMultiplier {
    fn multiply(&self, frequency: f32) -> f32 {
        match self {
            FrequencyMultiplier::OneEigth => frequency / 8.0,
            FrequencyMultiplier::OneFourth => frequency / 4.0,
            FrequencyMultiplier::OneHalf => frequency / 2.0,
            FrequencyMultiplier::One => frequency,
            FrequencyMultiplier::Two => frequency * 2.0,
            FrequencyMultiplier::Three => frequency * 3.0,
            FrequencyMultiplier::Four => frequency * 4.0,
            FrequencyMultiplier::Five => frequency * 5.0,
            FrequencyMultiplier::Six => frequency * 6.0,
            FrequencyMultiplier::Seven => frequency * 7.0,
            FrequencyMultiplier::Eight => frequency * 8.0,
            FrequencyMultiplier::Nine => frequency * 9.0,
            FrequencyMultiplier::Ten => frequency * 10.0,
        }
    }
}

impl Default for FrequencyMultiplier {
    fn default() -> Self {
        Self::One
    }
}

#[derive(Default, Clone)]
pub struct Operator {
    waveform: Waveform,
    max_level: u8,
    frequency_multiplier: FrequencyMultiplier,
    detune: i8,
}

impl Operator {
    pub fn func(&self, modulation: f32, tone: f32) -> f32 {
        self.waveform
            .func(modulation + self.frequency_multiplier.multiply(tone))
            * self.attenuation()
            * AMPLIFICATION
    }

    fn attenuation(&self) -> f32 {
        let db = -(ENV_DB / (u8::MAX as f32 + 1.0)) * (u8::MAX - self.max_level) as f32;
        10f32.powf(db / 20.0)
    }
}

#[derive(PartialEq, Clone)]
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

#[derive(Clone)]
pub struct Patch {
    pub(crate) active: bool,
    sample_rate: u32,
    pub(crate) clock: u32,
    pub(crate) base_frequency: f32,
    operators: [Operator; 4],
    algorithm: Algorithm,
    feedback: u8,
}

impl Patch {
    pub fn new(base_frequency: f32, sample_rate: u32) -> Self {
        Self {
            active: false,
            sample_rate,
            clock: 0,
            base_frequency,
            operators: [
                Operator {
                    waveform: Waveform::Sine,
                    max_level: 0,
                    frequency_multiplier: FrequencyMultiplier::OneEigth,
                    detune: 0,
                },
                Operator {
                    waveform: Waveform::Sine,
                    max_level: 186,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                },
                Operator {
                    waveform: Waveform::Sine,
                    max_level: 200,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                },
                Operator {
                    waveform: Waveform::Sine,
                    max_level: 255,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                },
            ],
            algorithm: Algorithm::default(),
            feedback: 0,
        }
    }

    fn func(&self, base_tone: f32) -> f32 {
        // 1st operator is always feedback
        if self.feedback > 0 {
            unimplemented!()
        };

        if self.algorithm != Algorithm::One {
            unimplemented!()
        };

        self.operators.iter().fold(0.0, |accumulator, operator| {
            operator.func(accumulator, base_tone)
        }) / AMPLIFICATION
    }

    //TODO: Potentially add left/right scaling here?
    //Would it be better to do each operator and combine them later?
    pub(crate) fn write_to_buffer(&mut self, data: &mut [f32], channels: u16) {
        data.chunks_exact_mut(channels as usize)
            .zip(self)
            .for_each(|(frame, sample)| frame.iter_mut().for_each(|data| *data += sample))
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
