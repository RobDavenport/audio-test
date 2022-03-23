use std::collections::VecDeque;

const START_OCTAVE: usize = 4;
const START_NAME: usize = 9;
const NAMES: [&str; 12] = [
    "C ", "C#", "D ", "D#", "E ", "F ", "F#", "G ", "G#", "A ", "A#", "B ",
];
const ROOT_NOTE_FREQ: f64 = 440.0;

const HIGHEST_OCTAVE: usize = 9;
const HIGHEST_NOTE: usize = 0;

const LOWEST_OCTAVE: usize = 0;
const LOWEST_NOTE: usize = 11;

const TOTAL_NOTES: usize = ((HIGHEST_OCTAVE - LOWEST_OCTAVE) - 1) * NAMES.len();

static mut NOTES_TABLE: &mut [NoteEntry; TOTAL_NOTES] = &mut [NoteEntry {
    name: 0,
    octave: 0,
    frequency: 0.0,
}; TOTAL_NOTES];

#[derive(Clone, Copy)]
pub struct NoteEntry {
    name: usize,
    octave: usize,
    frequency: f32,
}

pub fn generate() {
    let mut notes = VecDeque::new();
    notes.push_back(NoteEntry {
        name: START_NAME,
        octave: START_OCTAVE,
        frequency: ROOT_NOTE_FREQ as f32,
    });
    let a = 2.0f64.powf(12.0_f64.recip());

    // Going up until B8
    let mut note_name = START_NAME + 1;
    let mut octave = START_OCTAVE;
    let mut note_delta = 1;

    while octave != HIGHEST_OCTAVE || note_name != HIGHEST_NOTE {
        let next_frequency = ROOT_NOTE_FREQ * a.powi(note_delta);
        println!(
            "appending: {}{}: {}",
            NAMES[note_name], octave, next_frequency
        );

        notes.push_back(NoteEntry {
            name: note_name,
            octave,
            frequency: next_frequency as f32,
        });

        note_name = note_name + 1;
        note_delta += 1;
        if note_name == 12 {
            note_name = 0;
            octave += 1;
        }
    }

    // Going down until C1
    let mut note_name = START_NAME - 1;
    let mut octave = START_OCTAVE;
    let mut note_delta = -1;

    while octave != LOWEST_OCTAVE || note_name != LOWEST_NOTE {
        let next_frequency = ROOT_NOTE_FREQ * a.powi(note_delta);
        println!(
            "inserting: {}{}: {}",
            NAMES[note_name], octave, next_frequency
        );
        notes.push_front(NoteEntry {
            name: note_name,
            octave,
            frequency: next_frequency as f32,
        });

        if note_name == 0 {
            note_name = 12;
            octave -= 1;
        }
        note_name = note_name - 1;
        note_delta -= 1;
    }

    unsafe {
        NOTES_TABLE
            .iter_mut()
            .zip(notes.into_iter())
            .for_each(|(table, generated)| *table = generated);

        println!(
            "generated {} notes from {:?} to {:?}",
            NOTES_TABLE.len(),
            index_to_name(0),
            index_to_name(NOTES_TABLE.len() - 1)
        );
    }
}

pub fn index_to_frequency(index: usize) -> f32 {
    unsafe { NOTES_TABLE[index].frequency }
}

pub fn index_to_name(index: usize) -> String {
    unsafe {
        let note = &NOTES_TABLE[index];
        format!("{}{}", NAMES[note.name], note.octave)
    }
}
