use std::{mem::MaybeUninit, sync::Arc};

use super::{
    Algorithm, EnvelopeDefinition, EnvelopeInstance, FeedbackLevel, FrequencyMultiplier,
    OperatorDefinition, OperatorInstance, OPERATOR_COUNT,
};
use crate::Waveform;

#[derive(Clone)]
pub struct PatchDefinition {
    pub(crate) operators: [Arc<OperatorDefinition>; OPERATOR_COUNT],
    pub(crate) algorithm: Algorithm,
    pub(crate) feedback: FeedbackLevel,
    pub(crate) wall_tick_time: f32,
}

impl PatchDefinition {
    pub(crate) fn generate_new_operators(&self) -> [OperatorInstance; OPERATOR_COUNT] {
        let mut output: [MaybeUninit<OperatorInstance>; OPERATOR_COUNT] =
            unsafe { MaybeUninit::uninit().assume_init() };

        self.operators
            .iter()
            .zip(output.iter_mut())
            .for_each(|(source, target)| {
                target.write(OperatorInstance {
                    definition: source.clone(),
                    envelope: EnvelopeInstance::new(source.envelope.clone()),
                });
            });

        unsafe { std::mem::transmute(output) }
    }
}

impl PatchDefinition {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            wall_tick_time: 1.0 / sample_rate as f32,
            operators: [
                Arc::new(OperatorDefinition {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::default()),
                }),
                Arc::new(OperatorDefinition {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::default()),
                }),
                Arc::new(OperatorDefinition {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::default()),
                }),
                Arc::new(OperatorDefinition {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::new(255, 150, 60, 0, 60, 60)),
                }),
            ],
            algorithm: Algorithm::One,
            feedback: FeedbackLevel::Seven,
        }
    }
}
