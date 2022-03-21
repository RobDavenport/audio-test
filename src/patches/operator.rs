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
    pub fn func(&self, modulation: f32, tone: f32) -> f32 {
        self.waveform
            .func(modulation + self.frequency_multiplier.multiply(tone))
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
    fn multiply(&self, frequency: f32) -> f32 {
        match self {
            FrequencyMultiplier::OneSixteenth => frequency / 16.0,
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
            FrequencyMultiplier::Eleven => frequency * 11.0,
            FrequencyMultiplier::Twelve => frequency * 12.0,
        }
    }
}