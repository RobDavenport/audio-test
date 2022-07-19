use std::sync::Arc;

use parking_lot::RwLock;

use crate::{Waveform, TARGET_SAMPLE_RATE, sin};

use super::{EnvelopeDefinition, EnvelopeInstance, FrequencyMultiplier};

// const ONE_SEMITONE: f32 = 2.0_f32.powf(1.0/12.0);

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
    pub(crate) clock: u32,
}

impl OperatorInstance {
    pub fn func(&mut self, base_frequency: f32, modulation: f32) -> f32 {
        let definition = self.definition.read();

        let frequency =
            definition.frequency_multiplier.multiply(base_frequency) * self.detune_as_multiplier();
        self.clock = self.clock.wrapping_add(sin::get_delta_p(frequency));
        // let amt = TARGET_SAMPLE_RATE as f32 / frequency;

        // self.clock += 1.0;
        // if self.clock > amt {
        //     self.clock -= amt
        // }

        definition.waveform.func(self.clock, modulation) * self.envelope.attenuation()
    }

    fn detune_as_multiplier(&self) -> f32 {
        let detune = self.definition.read().detune;
        assert!(detune <= 100);
        assert!(detune >= -100);
        if detune >= 0 {
            1.0 + ((detune as f32 / 100.0) * 0.059_463_095)
        } else {
            1.0 + ((detune as f32 / 100.0) * (1.0 - 0.943_874_3))
        }
    }
}
