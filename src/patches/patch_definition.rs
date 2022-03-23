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
            operators: [
                Arc::new(OperatorDefinition {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::new(205, 21, 7, 0, 170, 170)),
                }),
                Arc::new(OperatorDefinition {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::new(125, 255, 0, 0, 80, 80)),
                }),
                Arc::new(OperatorDefinition {
                    waveform: Waveform::Saw,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::new(215, 255, 220, 0, 0, 0)),
                }),
                Arc::new(OperatorDefinition {
                    waveform: Waveform::CamelSine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Arc::new(EnvelopeDefinition::new(255, 255, 50, 0, 129, 129)),
                }),
            ],
            // operators: [
            //     Arc::new(OperatorDefinition {
            //         waveform: Waveform::AbsoluteSine,
            //         frequency_multiplier: FrequencyMultiplier::One,
            //         detune: 0,
            //         envelope: Arc::new(EnvelopeDefinition::new(255, 252, 5, 25, 120, 140)),
            //     }),
            //     Arc::new(OperatorDefinition {
            //         waveform: Waveform::CamelSine,
            //         frequency_multiplier: FrequencyMultiplier::OneHalf,
            //         detune: 0,
            //         envelope: Arc::new(EnvelopeDefinition::new(210, 132, 050, 250, 141, 200)),
            //     }),
            //     Arc::new(OperatorDefinition {
            //         waveform: Waveform::Sine,
            //         frequency_multiplier: FrequencyMultiplier::OneHalf,
            //         detune: 0,
            //         envelope: Arc::new(EnvelopeDefinition::new(235, 245, 130, 250, 122, 160)),
            //     }),
            //     Arc::new(OperatorDefinition {
            //         waveform: Waveform::Sine,
            //         frequency_multiplier: FrequencyMultiplier::One,
            //         detune: 0,
            //         envelope: Arc::new(EnvelopeDefinition::new(255, 195, 25, 220, 152, 180)),
            //     }),
            // ],
            algorithm: Algorithm::One,
            feedback: FeedbackLevel::Zero,
        }
    }
}
