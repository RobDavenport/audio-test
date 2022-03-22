pub struct Pattern {
    entires: Box<[PatternEntry]>,
}

// TODO: How to handle effects?
// TOOD: How to handle repeat points on patterns?
pub(crate) struct PatternEntry {
    patch_index: Option<usize>,
    key_state: Option<usize>,
}
