use std::sync::Arc;

use parking_lot::RwLock;

use crate::{Waveform, TARGET_SAMPLE_RATE};

use super::{EnvelopeDefinition, EnvelopeInstance};

#[derive(Default, Clone, Debug)]
pub struct OperatorDefinition {
    pub(crate) waveform: Waveform,
    pub(crate) frequency_multiplier: FrequencyMultiplier,
    pub(crate) detune: i8,
    pub(crate) envelope: Arc<RwLock<EnvelopeDefinition>>,
}

pub struct OperatorInstance {
    pub(crate) definition: Arc<RwLock<OperatorDefinition>>,
    pub(crate) envelope: EnvelopeInstance,
    pub(crate) clock: f32,
}

impl OperatorInstance {
    pub fn func(&mut self, frequency: f32, modulation: f32) -> f32 {
        let definition = self.definition.read();

        let frequency = definition.frequency_multiplier.multiply(frequency);
        self.clock = (self.clock + 1.0) % (TARGET_SAMPLE_RATE as f32 / frequency);

        definition.waveform.func(self.clock, frequency, modulation) * self.envelope.attenuation()
    }
}

#[derive(Clone, Debug)]
pub struct FrequencyMultiplier(pub u8);

impl Default for FrequencyMultiplier {
    fn default() -> Self {
        Self(6)
    }
}

// Carrier: Modulator
impl FrequencyMultiplier {
    pub fn max_value() -> u8 {
        20
    }

    pub fn as_ratio(&self) -> &str {
        match self.0 {
            0 => "4:1 0.25",
            1 => "3:1 ~0.33333",
            2 => "8:3 ~0.375",
            3 => "2:1 0.5",
            4 => "3:2 ~0.666",
            5 => "4:3 ~0.75",
            6 => "1:1 1.0",
            7 => "4:5 1.25",
            8 => "3:4 ~1.33",
            9 => "2:3 1.5",
            10 => "3:5 ~1.66",
            11 => "1:2 2.0",
            12 => "2:5 2.5",
            13 => "3:8 ~2.666",
            14 => "1:3 3.0",
            15 => "3:10 ~3.333",
            16 => "1:4 4.0",
            17 => "1:5 5.0",
            18 => "3:16 ~5.333",
            19 => "1:6 6.0",
            20 => "3:20 ~6.666",
            _ => panic!("invalid frequency multiplier value"),
        }
    }

    fn multiply(&self, phase: f32) -> f32 {
        match self.0 {
            0 => phase * (1. / 4.),   // 4:1 0.25
            1 => phase * (1. / 3.),   // 3:1 ~0.33333
            2 => phase * (3. / 8.),   // 8:3 ~0.375
            3 => phase * (1. / 2.),   // 2:1 0.5
            4 => phase * (2. / 3.),   // 3:2 ~0.666
            5 => phase * (3. / 4.),   // 4:3 ~0.75
            6 => phase,               // 1:1
            7 => phase * (5. / 4.),   // 4:5 1.25
            8 => phase * (4. / 3.),   // 3:4  ~1.33
            9 => phase * (3. / 2.),   // 2:3  1.5
            10 => phase * (5. / 3.),  // 3:5  ~1.66
            11 => phase * 2.0,        // 1:2 2.0
            12 => phase * (5. / 2.),  // 2:5  2.5
            13 => phase * (8. / 3.),  // 3:8  ~2.666
            14 => phase * 3.0,        // 1:3 3.0
            15 => phase * (10. / 3.), // 3:10 ~3.333
            16 => phase * 4.0,        // 1:4 4.0
            17 => phase * (5. / 1.),  // 1:5  5.0
            18 => phase * (16. / 3.), // 3:16 ~5.333
            19 => phase * (6. / 1.),  // 1:6  6.0
            20 => phase * (20. / 3.), // 3:20 ~6.666
            _ => panic!("invalid frequency multiplier value"),
        }
    }
}
