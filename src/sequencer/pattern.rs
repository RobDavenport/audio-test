#[derive(Debug, Clone)]
pub struct Pattern {
    pub(crate) entires: Box<[PatternEntry]>,
}

impl Pattern {
    pub fn pattern_length(&self) -> usize {
        self.entires.len()
    }

    pub fn empty_pattern(len: usize) -> Self {
        Self {
            entires: (0..len)
                .map(|_| PatternEntry {
                    patch_index: None,
                    key_state: KeyState::Released,
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

// TODO: How to handle effects?
// TOOD: How to handle repeat points on patterns?
#[derive(Debug, Clone)]
pub(crate) struct PatternEntry {
    pub(crate) patch_index: Option<usize>,
    pub(crate) key_state: KeyState,
}

#[derive(Debug, Clone)]
pub enum KeyState {
    Released,
    Held,
    Slide(usize),
    Pressed(usize),
}
