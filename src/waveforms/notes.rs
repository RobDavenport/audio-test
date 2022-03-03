use std::{collections::VecDeque, sync::Arc};

const START_OCTAVE: usize = 4;
const NAMES: [&str; 12] = [
    "C ", "C# ", "D ", "D#", "E ", "F ", "F#", "G ", "G#", "A ", "A#", "B ",
];
const ROOT_NOTE_FREQ: f64 = 440.0;
const ROOT_NOTE_NAME: usize = 9;

#[derive(Clone)]
pub struct Notes {
    notes: Arc<Box<[(f32, String)]>>,
}

impl Notes {
    pub fn generate() -> Self {
        let mut notes = VecDeque::new();
        notes.push_back((ROOT_NOTE_FREQ, format!("{}{}", NAMES[0], START_OCTAVE)));
        let a = 2.0f64.powf(12.0_f64.recip());

        // Going up until B8
        let mut note_name = 10;
        let mut octave = START_OCTAVE;
        let mut note_delta = 1;

        while octave != 9 {
            let next_frequency = ROOT_NOTE_FREQ * a.powi(note_delta);
            let next_note_name = format!("{}{}", NAMES[note_name], octave);
            println!("appending: {}: {}", next_note_name, next_frequency);

            notes.push_back((next_frequency, next_note_name));

            note_name = note_name + 1;
            note_delta += 1;
            if note_name == 12 {
                note_name = 0;
                octave += 1;
            }
        }

        // Going down until C1
        let mut note_name = 8;
        let mut octave = START_OCTAVE;
        let mut note_delta = -1;

        while octave != 0 || note_name != 11 {
            let next_frequency = ROOT_NOTE_FREQ * a.powi(note_delta);
            let next_note_name = format!("{}{}", NAMES[note_name], octave);
            println!("inserting: {}: {}", next_note_name, next_frequency);
            notes.push_front((next_frequency, next_note_name));

            if note_name == 0 {
                note_name = 12;
                octave -= 1;
            }
            note_name = note_name - 1;
            note_delta -= 1;
        }

        println!(
            "generated {} notes from {:?} to {:?}",
            notes.len(),
            notes[0],
            notes[notes.len() - 1]
        );

        let notes = notes
            .into_iter()
            .map(|(freq, string)| (freq as f32, string))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            notes: Arc::new(notes),
        }
    }

    pub fn from_index(&self, index: usize) -> &(f32, String) {
        &self.notes[index]
    }

    pub fn index_to_frequency(&self, index: usize) -> f32 {
        self.notes[index].0
    }

    pub fn index_to_name(&self, index: usize) -> &str {
        &self.notes[index].1
    }
}
