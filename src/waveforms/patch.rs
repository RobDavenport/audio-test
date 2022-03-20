use std::{
    f32::consts::{PI, TAU},
    sync::Arc,
};

use parking_lot::RwLock;

use super::oscillator::Waveform;

const AMPLIFICATION: f32 = 25.0;
const ENV_DB: f32 = 96.0;
//const ENV_DB: f32 = 64.0;

const OPERATOR_COUNT: usize = 4;

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

    pub fn set_active(&self, active: bool) {
        self.patch.write().active = active;
    }

    pub fn write_to_buffer(&mut self, data: &mut [f32], channels: u16) {
        let mut lock = self.patch.write();
        lock.write_to_buffer(data, channels)
    }
}

#[derive(Clone)]
pub enum FrequencyMultiplier {
    OneSixteenth,
    OneEigth,
    OneFourth,
    OneHalf,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl FrequencyMultiplier {
    fn multiply(&self, frequency: f32) -> f32 {
        match self {
            FrequencyMultiplier::OneSixteenth => frequency / 16.0,
            FrequencyMultiplier::OneEigth => frequency / 8.0,
            FrequencyMultiplier::OneFourth => frequency / 4.0,
            FrequencyMultiplier::OneHalf => frequency / 2.0,
            FrequencyMultiplier::One => frequency,
            FrequencyMultiplier::Two => frequency * 2.0,
            FrequencyMultiplier::Three => frequency * 3.0,
            FrequencyMultiplier::Four => frequency * 4.0,
            FrequencyMultiplier::Five => frequency * 5.0,
            FrequencyMultiplier::Six => frequency * 6.0,
            FrequencyMultiplier::Seven => frequency * 7.0,
            FrequencyMultiplier::Eight => frequency * 8.0,
            FrequencyMultiplier::Nine => frequency * 9.0,
            FrequencyMultiplier::Ten => frequency * 10.0,
            FrequencyMultiplier::Eleven => frequency * 11.0,
            FrequencyMultiplier::Twelve => frequency * 12.0,
        }
    }
}

#[derive(Clone)]
pub enum FeedbackLevel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

impl Default for FeedbackLevel {
    fn default() -> Self {
        Self::Zero
    }
}

impl FeedbackLevel {
    pub fn as_multiplier(&self) -> f32 {
        match self {
            FeedbackLevel::Zero => 0.0,
            FeedbackLevel::One => PI / 128.0,
            FeedbackLevel::Two => PI / 64.0,
            FeedbackLevel::Three => PI / 32.0,
            FeedbackLevel::Four => PI / 16.0,
            FeedbackLevel::Five => PI / 8.0,
            FeedbackLevel::Six => PI / 4.0,
            FeedbackLevel::Seven => PI / 2.0,
            FeedbackLevel::Eight => PI,
            FeedbackLevel::Nine => PI * 2.0,
            FeedbackLevel::Ten => PI * 4.0,
            FeedbackLevel::Eleven => PI * 8.0,
            FeedbackLevel::Twelve => PI * 16.0,
            FeedbackLevel::Thirteen => PI * 32.0,
            FeedbackLevel::Fourteen => PI * 64.0,
            FeedbackLevel::Fifteen => PI * 128.0,
        }
    }
}

impl Default for FrequencyMultiplier {
    fn default() -> Self {
        Self::One
    }
}

#[derive(Default, Clone)]
pub struct Operator {
    waveform: Waveform,
    max_level: u8,
    frequency_multiplier: FrequencyMultiplier,
    detune: i8,
}

impl Operator {
    pub fn func(&self, modulation: f32, tone: f32) -> f32 {
        self.waveform
            .func(modulation + self.frequency_multiplier.multiply(tone))
            * self.attenuation()
    }

    fn attenuation(&self) -> f32 {
        let db = -(ENV_DB / (u8::MAX as f32 + 1.0)) * (u8::MAX - self.max_level) as f32;
        10f32.powf(db / 20.0)
    }
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

pub struct AlgorithmDefinition {
    carriers: [bool; OPERATOR_COUNT],
    modulators: [ModulatedBy; OPERATOR_COUNT],
}

pub enum ModulatedBy {
    None,
    Feedback,
    Single(usize),
    Double(usize, usize),
}

impl Algorithm {
    pub fn get_definition(&self) -> &'static AlgorithmDefinition {
        match self {
            Algorithm::Zero => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Feedback,
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(1),
                    ModulatedBy::Single(2),
                ],
            },
            Algorithm::One => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Feedback,
                    ModulatedBy::None,
                    ModulatedBy::Double(0, 1),
                    ModulatedBy::Single(2),
                ],
            },
            Algorithm::Two => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Feedback,
                    ModulatedBy::None,
                    ModulatedBy::Single(1),
                    ModulatedBy::Single(2),
                ],
            },
            Algorithm::Three => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Feedback,
                    ModulatedBy::Single(0),
                    ModulatedBy::None,
                    ModulatedBy::Double(1, 2),
                ],
            },
            Algorithm::Four => &AlgorithmDefinition {
                carriers: [false, true, false, true],
                modulators: [
                    ModulatedBy::Feedback,
                    ModulatedBy::Single(0),
                    ModulatedBy::None,
                    ModulatedBy::Single(3),
                ],
            },
            Algorithm::Five => &AlgorithmDefinition {
                carriers: [false, true, true, true],
                modulators: [
                    ModulatedBy::Feedback,
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(0),
                ],
            },
            Algorithm::Six => &AlgorithmDefinition {
                carriers: [false, true, true, true],
                modulators: [
                    ModulatedBy::Feedback,
                    ModulatedBy::Single(0),
                    ModulatedBy::None,
                    ModulatedBy::None,
                ],
            },
            Algorithm::Seven => &AlgorithmDefinition {
                carriers: [true, true, true, true],
                modulators: [
                    ModulatedBy::Feedback,
                    ModulatedBy::None,
                    ModulatedBy::None,
                    ModulatedBy::None,
                ],
            },
        }
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Self::One
    }
}

#[derive(Clone)]
pub struct Patch {
    pub(crate) active: bool,
    sample_rate: u32,
    pub(crate) clock: u32,
    pub(crate) base_frequency: f32,
    operators: [Operator; 4],
    algorithm: Algorithm,
    prev_feedback1: f32,
    prev_feedback2: f32,
    feedback: FeedbackLevel,
}

impl Patch {
    pub fn new(base_frequency: f32, sample_rate: u32) -> Self {
        Self {
            active: false,
            sample_rate,
            clock: 0,
            base_frequency,
            prev_feedback1: 0.0,
            prev_feedback2: 0.0,
            operators: [
                Operator {
                    waveform: Waveform::Sine,
                    max_level: 255,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                },
                Operator {
                    waveform: Waveform::Sine,
                    max_level: 0,
                    frequency_multiplier: FrequencyMultiplier::OneHalf,
                    detune: 0,
                },
                Operator {
                    waveform: Waveform::Sine,
                    max_level: 200,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                },
                Operator {
                    waveform: Waveform::Sine,
                    max_level: 255,
                    frequency_multiplier: FrequencyMultiplier::One,
                    detune: 0,
                },
            ],
            algorithm: Algorithm::Seven,
            feedback: FeedbackLevel::Seven,
        }
    }

    fn func(&mut self, base_tone: f32) -> f32 {
        // 1st operator is always feedback
        let mut outputs = [0.0f32; 4];
        let mut final_output = 0.0f32;

        let algorithm = self.algorithm.get_definition();

        (0..OPERATOR_COUNT).for_each(|i| {
            let result = match algorithm.modulators[i] {
                ModulatedBy::None => self.operators[i].func(0.0, base_tone),
                ModulatedBy::Feedback => self.operators[i].func(
                    ((self.prev_feedback1 + self.prev_feedback2) / 2.0)
                        * self.feedback.as_multiplier(),
                    base_tone,
                ),
                ModulatedBy::Single(modulator) => {
                    self.operators[i].func(outputs[modulator], base_tone)
                }
                ModulatedBy::Double(first, second) => {
                    self.operators[i].func(outputs[first] + outputs[second], base_tone)
                }
            };

            let result = result * AMPLIFICATION;

            outputs[i] = result;

            if algorithm.carriers[i] == true {
                final_output += result;
            }
        });

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
        if self.active {
            self.clock += 1;

            let tone = self.clock as f32 * self.base_frequency * TAU / self.sample_rate as f32;

            Some(self.func(tone))
        } else {
            None
        }
    }
}
