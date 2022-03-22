use std::sync::Arc;

use crate::{PatchDefinition, PatchInstance, TARGET_SAMPLE_TICK_TIME};

use super::{Pattern, CHANNEL_COUNT};

#[derive(Clone)]
pub struct SequenceDefinition {
    bpm: f32,
    patches: Arc<Box<[PatchDefinition]>>, // The available patches
    patterns: Arc<[Pattern; CHANNEL_COUNT]>, // The notes played
    wall_tick_time: f32,
}

pub struct SequenceInstance {
    definition: SequenceDefinition,
    output: [PatchInstance; CHANNEL_COUNT],
    wall_clock: f32,
    last_output: f32,
}

impl Iterator for SequenceInstance {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.wall_clock += self.definition.wall_tick_time;

        //TODO: Could optimize this with integer math?
        if self.wall_clock >= TARGET_SAMPLE_TICK_TIME {
            self.wall_clock -= TARGET_SAMPLE_TICK_TIME;

            //TODO
            // If we should advance the pattern...
            // Read patterns and adjust accordingly

            // Produce sound
            self.last_output = self.output.iter_mut().fold(0.0, |accumulator, patch| {
                accumulator + patch.next().unwrap()
            });
        }

        Some(self.last_output)
    }
}
