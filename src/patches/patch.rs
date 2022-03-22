use std::{
    f32::consts::{PI, TAU},
    sync::Arc,
};

use parking_lot::RwLock;

use super::{
    Algorithm, Envelope, FeedbackLevel, FrequencyMultiplier, ModulatedBy, Operator, AMPLIFICATION,
    OPERATOR_COUNT,
};
use crate::{Waveform, TARGET_SAMPLE_RATE, TARGET_SAMPLE_TICK_TIME};

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

    pub fn set_algorithm(&self, algorithm: Algorithm) {
        self.patch.write().algorithm = algorithm
    }

    pub fn set_waveform(&self, operator_index: usize, waveform: Waveform) {
        self.patch.write().operators[operator_index].waveform = waveform;
    }

    pub fn set_active(&self, active: bool) {
        let lock = &mut self.patch.write();

        if active != lock.active {
            lock.active = active;
            match active {
                true => {
                    println!("key on!");
                    lock.operators
                        .iter_mut()
                        .for_each(|operator| operator.envelope.key_on())
                }
                false => {
                    println!("key off!");
                    lock.operators
                        .iter_mut()
                        .for_each(|operator| operator.envelope.key_off())
                }
            }
        }
    }

    pub fn write_to_buffer(&mut self, data: &mut [f32], channels: u16) {
        let mut lock = self.patch.write();
        lock.write_to_buffer(data, channels)
    }
}

#[derive(Clone)]
pub struct Patch {
    pub(crate) active: bool,
    pub(crate) clock: u32,
    pub(crate) base_frequency: f32,
    operators: [Operator; 4],
    algorithm: Algorithm,
    prev_feedback1: f32,
    prev_feedback2: f32,
    feedback: FeedbackLevel,
    wall_clock: f32,
    wall_tick_time: f32,
}

impl Patch {
    pub fn new(base_frequency: f32, sample_rate: u32) -> Self {
        Self {
            active: false,
            clock: 0,
            wall_clock: 0.0,
            base_frequency,
            prev_feedback1: 0.0,
            prev_feedback2: 0.0,
            wall_tick_time: 1.0 / sample_rate as f32,
            operators: [
                Operator {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Envelope::default(),
                },
                Operator {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Envelope::default(),
                    //envelope: Envelope::new(170, 90, 80, 0, 40, 40),
                },
                Operator {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Envelope::default(),
                    //envelope: Envelope::new(205, 230, 80, 0, 90, 90),
                },
                Operator {
                    waveform: Waveform::Sine,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                    envelope: Envelope::new(255, 150, 60, 0, 60, 60),
                },
            ],
            algorithm: Algorithm::One,
            feedback: FeedbackLevel::Seven,
        }
    }

    fn func(&mut self, phase: f32) -> f32 {
        let mut outputs = [0.0f32; 4];
        let mut final_output = 0.0f32;

        let algorithm = self.algorithm.get_definition();

        // 1st Operator is always feedback
        outputs[0] = self.operators[0].func(
            (((self.prev_feedback1 + self.prev_feedback2) / 2.0) * self.feedback.as_multiplier())
                + phase,
        );

        if algorithm.carriers[0] == true {
            final_output += outputs[0];
        };
        // End 1st Operator

        (1..OPERATOR_COUNT).for_each(|i| {
            let result = match algorithm.modulators[i - 1] {
                ModulatedBy::None => self.operators[i].func(phase),
                ModulatedBy::Single(modulator) => {
                    self.operators[i].func(outputs[modulator] + phase)
                }
                ModulatedBy::Double(first, second) => {
                    self.operators[i].func(outputs[first] + outputs[second] + phase)
                }
            };

            let result = result * AMPLIFICATION;

            outputs[i] = result;

            if algorithm.carriers[i] == true {
                final_output += result;
            }
        });

        // Handle feedback
        self.prev_feedback2 = self.prev_feedback1;
        let final_output = final_output / AMPLIFICATION;
        self.prev_feedback1 = final_output;
        final_output
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
        self.wall_clock += self.wall_tick_time;

        //TODO: Could optimize this with integer math?
        while self.wall_clock >= TARGET_SAMPLE_TICK_TIME {
            self.clock += 1;
            self.wall_clock -= TARGET_SAMPLE_TICK_TIME;

            self.operators
                .iter_mut()
                .for_each(|operator| operator.envelope.tick());
        }

        let phase = self.clock as f32 * self.base_frequency * TAU / TARGET_SAMPLE_RATE as f32;

        Some(self.func(phase))
    }
}
