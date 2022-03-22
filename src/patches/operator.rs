use crate::Waveform;

use super::Envelope;

#[derive(Default, Clone)]
pub struct Operator {
    pub(crate) waveform: Waveform,
    pub(crate) frequency_multiplier: FrequencyMultiplier,
    pub(crate) detune: i8,
    pub(crate) envelope: Envelope,
}

impl Operator {
    pub fn func(&self, modulation: f32, phase: f32) -> f32 {
        self.waveform
            .func(modulation + self.frequency_multiplier.multiply(phase))
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
