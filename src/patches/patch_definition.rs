use std::{mem::MaybeUninit, sync::Arc};

use super::{
    Algorithm, EnvelopeDefinition, EnvelopeInstance, FeedbackLevel, FrequencyMultiplier,
    OperatorDefinition, OperatorInstance, OPERATOR_COUNT,
};
use crate::Waveform;

#[derive(Clone, Debug)]
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
            // operators: [
            //     Arc::new(OperatorDefinition {
            //         waveform: Waveform::Sine,
            //         frequency_multiplier: FrequencyMultiplier::One,
            //         detune: 0,
            //         envelope: Arc::new(EnvelopeDefinition::new(255, 255, 0, 255, 0, 255)),
            //     }),
            //     Arc::new(OperatorDefinition {
            //         waveform: Waveform::Sine,
            //         frequency_multiplier: FrequencyMultiplier::One,
            //         detune: 0,
            //         envelope: Arc::new(EnvelopeDefinition::default()),
            //     }),
            //     Arc::new(OperatorDefinition {
            //         waveform: Waveform::CamelSine,
            //         frequency_multiplier: FrequencyMultiplier::OneHalf,
            //         detune: 0,
            //         envelope: Arc::new(EnvelopeDefinition::default()),
            //     }),
            //     Arc::new(OperatorDefinition {
            //         waveform: Waveform::Sine,
            //         frequency_multiplier: FrequencyMultiplier::One,
            //         detune: 0,
            //         envelope: Arc::new(EnvelopeDefinition::default()),
            //     }),
            // ],
            operators: [
                Arc::new(OperatorDefinition {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::new(105, 135, 140, 250, 20, 50)),
                }),
                Arc::new(OperatorDefinition {
                    waveform: Waveform::QuarterSine,
                    frequency_multiplier: FrequencyMultiplier::Two,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::new(225, 135, 140, 250, 20, 200)),
                }),
                Arc::new(OperatorDefinition {
                    waveform: Waveform::CamelSine,
                    frequency_multiplier: FrequencyMultiplier::Two,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::new(220, 235, 140, 250, 040, 080)),
                }),
                Arc::new(OperatorDefinition {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::new(255, 249, 100, 240, 100, 120)),
                }),
            ],
            algorithm: Algorithm::One,
            feedback: FeedbackLevel::Nine,
        }
    }
}
