use std::sync::Arc;

use crate::{PatchDefinition, PatchInstance};

use super::{Pattern, CHANNEL_COUNT};

#[derive(Clone)]
pub struct SequenceDefinition {
    bpm: f32,
    patches: Arc<Box<[PatchDefinition]>>, // The available patches
    patterns: Arc<[Pattern; CHANNEL_COUNT]>, // The notes played
}

pub struct SequenceInstance {
    definition: SequenceDefinition,
    output: [PatchInstance; CHANNEL_COUNT],
}

impl Iterator for SequenceInstance {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
