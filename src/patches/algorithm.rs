use super::OPERATOR_COUNT;

pub enum ModulatedBy {
    None,
    Single(usize),
    Double(usize, usize),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Algorithm {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl Algorithm {
    pub fn get_definition(&self) -> &'static AlgorithmDefinition {
        match self {
            Algorithm::Zero => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(1),
                    ModulatedBy::Single(2),
                ],
            },
            Algorithm::One => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::None,
                    ModulatedBy::Double(0, 1),
                    ModulatedBy::Single(2),
                ],
            },
            Algorithm::Two => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::None,
                    ModulatedBy::Single(1),
                    ModulatedBy::Single(2),
                ],
            },
            Algorithm::Three => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::None,
                    ModulatedBy::Double(1, 2),
                ],
            },
            Algorithm::Four => &AlgorithmDefinition {
                carriers: [false, true, false, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::None,
                    ModulatedBy::Single(3),
                ],
            },
            Algorithm::Five => &AlgorithmDefinition {
                carriers: [false, true, true, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(0),
                ],
            },
            Algorithm::Six => &AlgorithmDefinition {
                carriers: [false, true, true, true],
                modulators: [ModulatedBy::Single(0), ModulatedBy::None, ModulatedBy::None],
            },
            Algorithm::Seven => &AlgorithmDefinition {
                carriers: [true, true, true, true],
                modulators: [ModulatedBy::None, ModulatedBy::None, ModulatedBy::None],
            },
        }
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Self::One
    }
}

pub struct AlgorithmDefinition {
    pub(crate) carriers: [bool; OPERATOR_COUNT],
    pub(crate) modulators: [ModulatedBy; OPERATOR_COUNT - 1],
}
