pub struct Pattern {
    entires: Box<[PatternEntry]>,
}

pub(crate) struct PatternEntry {
    patch_index: Option<usize>,
    key_state: Option<usize>,
}
