use std::sync::Arc;

use parking_lot::RwLock;

use crate::Waveform;

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
}

impl OperatorInstance {
    pub fn func(&self, phase: f32) -> f32 {
        let definition = self.definition.read();
        definition
            .waveform
            .func(definition.frequency_multiplier.multiply(phase))
            * self.envelope.attenuation()
    }
}

// #[derive(Clone, Debug)]
// pub enum FrequencyMultiplier {
//     OneSixteenth,
//     OneEigth,
//     OneFourth,
//     OneHalf,
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
//     Nine,
//     Ten,
//     Eleven,
//     Twelve,
// }

#[derive(Clone, Debug)]
pub struct FrequencyMultiplier(pub u8);

impl Default for FrequencyMultiplier {
    fn default() -> Self {
        Self(4)
    }
}

impl FrequencyMultiplier {
    fn multiply(&self, phase: f32) -> f32 {
        match self.0 {
            0 => phase / 16.0,
            1 => phase / 8.0,
            2 => phase / 4.0,
            3 => phase / 2.0,
            4 => phase,
            5 => phase * 2.0,
            6 => phase * 3.0,
            7 => phase * 4.0,
            8 => phase * 5.0,
            9 => phase * 6.0,
            10 => phase * 7.0,
            11 => phase * 8.0,
            12 => phase * 9.0,
            13 => phase * 10.0,
            14 => phase * 11.0,
            15 => phase * 12.0,
            _ => panic!("invalid frequency multiplier value"),
        }
    }
}
