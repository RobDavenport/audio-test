use std::sync::Arc;

use crate::Waveform;

use super::{EnvelopeDefinition, EnvelopeInstance};

#[derive(Default, Clone)]
pub struct OperatorDefinition {
    pub(crate) waveform: Waveform,
    pub(crate) frequency_multiplier: FrequencyMultiplier,
    pub(crate) detune: i8,
    pub(crate) envelope: Arc<EnvelopeDefinition>,
}

pub struct OperatorInstance {
    pub(crate) definition: Arc<OperatorDefinition>,
    pub(crate) envelope: EnvelopeInstance,
}

impl OperatorInstance {
    pub fn func(&self, phase: f32) -> f32 {
        self.definition
            .waveform
            .func(self.definition.frequency_multiplier.multiply(phase))
            * self.envelope.attenuation()
    }
}

#[derive(Clone)]
pub enum FrequencyMultiplier {
    OneSixteenth,
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
    Eleven,
    Twelve,
}

impl Default for FrequencyMultiplier {
    fn default() -> Self {
        Self::One
    }
}

impl FrequencyMultiplier {
    fn multiply(&self, phase: f32) -> f32 {
        match self {
            FrequencyMultiplier::OneSixteenth => phase / 16.0,
            FrequencyMultiplier::OneEigth => phase / 8.0,
            FrequencyMultiplier::OneFourth => phase / 4.0,
            FrequencyMultiplier::OneHalf => phase / 2.0,
            FrequencyMultiplier::One => phase,
            FrequencyMultiplier::Two => phase * 2.0,
            FrequencyMultiplier::Three => phase * 3.0,
            FrequencyMultiplier::Four => phase * 4.0,
            FrequencyMultiplier::Five => phase * 5.0,
            FrequencyMultiplier::Six => phase * 6.0,
            FrequencyMultiplier::Seven => phase * 7.0,
            FrequencyMultiplier::Eight => phase * 8.0,
            FrequencyMultiplier::Nine => phase * 9.0,
            FrequencyMultiplier::Ten => phase * 10.0,
            FrequencyMultiplier::Eleven => phase * 11.0,
            FrequencyMultiplier::Twelve => phase * 12.0,
        }
    }
}
