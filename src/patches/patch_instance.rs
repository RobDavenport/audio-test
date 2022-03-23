use std::{f32::consts::TAU, sync::Arc};

use parking_lot::RwLock;

use super::{ModulatedBy, OperatorInstance, PatchDefinition, AMPLIFICATION, OPERATOR_COUNT};
use crate::{TARGET_SAMPLE_RATE, TARGET_SAMPLE_TICK_TIME};

#[derive(Clone)]
pub struct PatchInstanceHandle {
    pub patch: Arc<RwLock<PatchInstance>>,
}

impl PatchInstanceHandle {
    pub fn new(patch: PatchInstance) -> Self {
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

    pub fn set_active(&self, active: bool) {
        self.patch.write().set_active(active);
    }

    pub fn write_to_buffer(&mut self, data: &mut [f32], channels: u16) {
        let mut lock = self.patch.write();
        lock.write_to_buffer(data, channels)
    }
}

pub struct PatchInstance {
    pub(crate) definition: Arc<PatchDefinition>,
    pub(crate) operators: [OperatorInstance; OPERATOR_COUNT],
    pub(crate) active: bool,
    pub(crate) clock: f32,
    pub(crate) base_frequency: f32,
    prev_feedback1: f32,
    prev_feedback2: f32,
    wall_clock: f32,
}

impl PatchInstance {
    pub fn new(definition: Arc<PatchDefinition>, base_frequency: f32) -> Self {
        Self {
            operators: definition.generate_new_operators(),
            definition,
            active: false,
            clock: 0.0,
            wall_clock: 0.0,
            base_frequency,
            prev_feedback1: 0.0,
            prev_feedback2: 0.0,
        }
    }

    fn func(&mut self, phase: f32) -> f32 {
        let mut outputs = [0.0f32; 4];
        let mut final_output = 0.0f32;

        let algorithm = self.definition.algorithm.get_definition();

        // 1st Operator is always feedback
        outputs[0] = self.operators[0].func(
            (((self.prev_feedback1 + self.prev_feedback2) / 2.0)
                * self.definition.feedback.as_multiplier())
                + phase,
        );

        // Handle feedback
        self.prev_feedback2 = self.prev_feedback1;
        self.prev_feedback1 = outputs[0];

        if algorithm.carriers[0] == true {
            final_output += outputs[0] * AMPLIFICATION;
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

        let final_output = final_output / AMPLIFICATION;
        final_output
    }

    //TODO: Potentially add left/right scaling here?
    //Would it be better to do each operator and combine them later?
    pub(crate) fn write_to_buffer(&mut self, data: &mut [f32], channels: u16) {
        data.chunks_exact_mut(channels as usize)
            .zip(self)
            .for_each(|(frame, sample)| frame.iter_mut().for_each(|data| *data += sample))
    }

    /// Forcefully tick
    pub(crate) fn force_tick(&mut self) -> f32 {
        self.tick();

        let phase = self.clock as f32 * self.base_frequency * TAU / TARGET_SAMPLE_RATE as f32;

        self.func(phase)
    }

    fn tick(&mut self) {
        self.clock = (self.clock + 1.0) % (TARGET_SAMPLE_RATE as f32 / self.base_frequency);

        self.operators
            .iter_mut()
            .for_each(|operator| operator.envelope.tick());
    }

    pub fn set_active(&mut self, active: bool) {
        if active != self.active {
            self.active = active;
            match active {
                true => self
                    .operators
                    .iter_mut()
                    .for_each(|operator| operator.envelope.key_on()),
                false => self
                    .operators
                    .iter_mut()
                    .for_each(|operator| operator.envelope.key_off()),
            }
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        println!("Setting frequency: {}", frequency);
        self.base_frequency = frequency
    }
}

impl Iterator for PatchInstance {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.wall_clock += self.definition.wall_tick_time;

        //TODO: Could optimize this with integer math?
        while self.wall_clock >= TARGET_SAMPLE_TICK_TIME {
            self.tick();
            self.wall_clock -= TARGET_SAMPLE_TICK_TIME;
        }

        let phase = self.clock * self.base_frequency * TAU / TARGET_SAMPLE_RATE as f32;

        Some(self.func(phase))
    }
}
