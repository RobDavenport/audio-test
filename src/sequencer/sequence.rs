use std::{mem::MaybeUninit, sync::Arc};

use parking_lot::RwLock;

use crate::{
    notes::{self},
    sequencer::KeyState,
    PatchDefinition, PatchInstance, TARGET_SAMPLE_RATE, TARGET_SAMPLE_TICK_TIME,
};

use super::{Pattern, PatternEntry, ENTRIES_PER_BEAT, MUSIC_CHANNEL_COUNT};

#[derive(Clone)]
pub struct SequenceInstanceHandle {
    pub(crate) sequence: Arc<RwLock<SequenceInstance>>,
}

impl SequenceInstanceHandle {
    pub fn new(sequence: SequenceInstance) -> Self {
        Self {
            sequence: Arc::new(RwLock::new(sequence)),
        }
    }

    //TODO: Potentially add left/right scaling here?
    //Would it be better to do each operator and combine them later?
    pub(crate) fn write_to_buffer(&self, data: &mut [f32], channels: u16) {
        let mut lock = self.sequence.write();
        lock.write_to_buffer(data, channels)
    }
}

#[derive(Clone, Debug)]
pub struct SequenceDefinition {
    bpm: f32,
    patches: Box<[Arc<PatchDefinition>]>, // The available patches
    patterns: Arc<[Pattern; MUSIC_CHANNEL_COUNT]>, // The notes played
    ticks_per_pattern_step: u32, // How many ticks until we need to advance to the next pattern
}

impl SequenceDefinition {
    pub fn new(
        bpm: f32,
        patches: Box<[Arc<PatchDefinition>]>,
        patterns: Arc<[Pattern; MUSIC_CHANNEL_COUNT]>,
    ) -> Self {
        println!("generating sequence definition");
        let beats_per_second = bpm / 60.0;
        let beats_per_sample_rate = TARGET_SAMPLE_RATE as f32 / beats_per_second;
        let ticks_per_beat = beats_per_sample_rate / ENTRIES_PER_BEAT as f32;

        println!(
            "beats per second: {}, beats_per_sample_rate: {}, ticks_per_beat: {} ",
            beats_per_second, beats_per_sample_rate, ticks_per_beat
        );
        Self {
            bpm,
            patches,
            patterns,
            ticks_per_pattern_step: ticks_per_beat as u32,
        }
    }

    pub fn test_pattern(sample_rate: u32) -> Self {
        let patches = PatchDefinition::new(sample_rate);
        let mut patterns = Vec::new();

        patterns.push(Pattern {
            entires: vec![
                PatternEntry {
                    patch_index: Some(0),
                    key_state: KeyState::Pressed(25),
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Released,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Pressed(25),
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Released,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Pressed(21),
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Released,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Pressed(21),
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Released,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Pressed(23),
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Released,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Pressed(23),
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Released,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Pressed(23),
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Released,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Pressed(23),
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Held,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Released,
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Pressed(23),
                },
                PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Released,
                },
            ]
            .into_boxed_slice(),
        });

        let demo_length = patterns[0].pattern_length();

        (1..MUSIC_CHANNEL_COUNT).for_each(|_| patterns.push(Pattern::empty_pattern(demo_length)));

        let patterns: Box<[Pattern; MUSIC_CHANNEL_COUNT]> =
            patterns.into_boxed_slice().try_into().unwrap();

        println!("generated test pattern!");
        Self::new(
            120.0,
            vec![Arc::new(patches)].into_boxed_slice(),
            Arc::new(*patterns),
        )
    }
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

    //TODO: Potentially add left/right scaling here?
    //Would it be better to do each operator and combine them later?
    pub(crate) fn write_to_buffer(&mut self, data: &mut [f32], channels: u16) {
        data.chunks_exact_mut(channels as usize)
            .zip(self)
            .for_each(|(frame, sample)| frame.iter_mut().for_each(|data| *data += sample))
    }
}

impl Iterator for SequenceInstance {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.wall_clock += TARGET_SAMPLE_TICK_TIME;

        //TODO: Could optimize this with integer math?
        while self.wall_clock >= TARGET_SAMPLE_TICK_TIME {
            self.wall_clock -= TARGET_SAMPLE_TICK_TIME;
            self.clock += 1;

            // If we should advance the pattern...
            if self.clock == self.definition.ticks_per_pattern_step {
                println!("advancing the pattern");
                self.clock = 0;

                // Wrap around if too long
                if self.pattern_index == self.definition.patterns[0].pattern_length() {
                    self.pattern_index = 0;
                }

                // TODO: Read patterns and adjust accordingly
                self.definition
                    .patterns
                    .iter()
                    .enumerate()
                    .for_each(|(channel, pattern)| {
                        let pattern = &pattern.entires[self.pattern_index];

                        match (pattern.patch_index, self.output[channel].as_ref()) {
                            (Some(new_patch_index), Some(current_patch)) => {
                                if !Arc::ptr_eq(
                                    &current_patch.definition,
                                    &self.definition.patches[new_patch_index],
                                ) {
                                    self.output[channel] = Some(PatchInstance::new(
                                        self.definition.patches[new_patch_index].clone(),
                                        0.0,
                                    ));
                                } else {
                                    println!("SAME PATCH, NO CHANGE!");
                                }
                            }
                            (Some(new_patch_index), None) => {
                                self.output[channel] = Some(PatchInstance::new(
                                    self.definition.patches[new_patch_index].clone(),
                                    0.0,
                                ));
                            }
                            _ => (),
                        }

                        // Play the key
                        if let Some(ref mut output_patch) = self.output[channel] {
                            match pattern.key_state {
                                KeyState::Released => output_patch.set_active(false),
                                KeyState::Pressed(index) => {
                                    output_patch.set_active(false);
                                    output_patch.set_frequency(notes::index_to_frequency(index));
                                    output_patch.set_active(true);
                                }
                                KeyState::Held => (),
                                KeyState::Slide(index) => {
                                    output_patch.set_frequency(notes::index_to_frequency(index));
                                }
                            }
                        }
                    });

                //Advance the pattern
                self.pattern_index += 1;
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
