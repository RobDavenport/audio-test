use std::{mem::MaybeUninit, sync::Arc};

use crate::{PatchDefinition, PatchInstance, TARGET_SAMPLE_TICK_TIME};

use super::{Pattern, MUSIC_CHANNEL_COUNT};

#[derive(Clone)]
pub struct SequenceDefinition {
    bpm: f32,
    patches: Arc<Box<[PatchDefinition]>>, // The available patches
    patterns: Arc<[Pattern; MUSIC_CHANNEL_COUNT]>, // The notes played
    wall_tick_time: f32,
    ticks_per_pattern: u32,
}

pub struct SequenceInstance {
    definition: Arc<SequenceDefinition>,
    output: [Option<PatchInstance>; MUSIC_CHANNEL_COUNT],
    wall_clock: f32,
    last_output: f32,
    clock: u32,
    pattern_index: usize,
}

impl SequenceInstance {
    pub fn new(definition: Arc<SequenceDefinition>) -> Self {
        Self {
            definition,
            output: empty_outputs(),
            wall_clock: 0.0,
            last_output: 0.0,
            clock: 0,
            pattern_index: 0,
        }
    }
}

impl Iterator for SequenceInstance {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.wall_clock += self.definition.wall_tick_time;

        //TODO: Could optimize this with integer math?
        if self.wall_clock >= TARGET_SAMPLE_TICK_TIME {
            self.wall_clock -= TARGET_SAMPLE_TICK_TIME;
            self.clock += 1;

            // If we should advance the pattern...
            if self.clock == self.definition.ticks_per_pattern {
                self.clock = 0;
                self.pattern_index += 1;
                // TODO: Read patterns and adjust accordingly
            }

            // Produce sound
            self.last_output = self.output.iter_mut().fold(0.0, |accumulator, patch| {
                if let Some(patch) = patch {
                    accumulator + patch.force_tick()
                } else {
                    accumulator
                }
            });
        }

        Some(self.last_output)
    }
}

fn empty_outputs() -> [Option<PatchInstance>; MUSIC_CHANNEL_COUNT] {
    let mut output: [MaybeUninit<Option<PatchInstance>>; MUSIC_CHANNEL_COUNT] =
        unsafe { MaybeUninit::uninit().assume_init() };

    output.iter_mut().for_each(|target| {
        target.write(None);
    });

    unsafe { std::mem::transmute(output) }
}
