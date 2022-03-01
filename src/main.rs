mod waveforms;

use macroquad::prelude::*;
use waveforms::*;

use crate::waveforms::waveform::{Envelope as Envelope1, Waveform, WaveformType};

const DUTY: f64 = 0.25;

#[macroquad::main("audio-test")]
async fn main() {
    let envelope = Envelope1 {
        attack_time: 0.05,
        start_amplitude: 1.0,
        decay_time: 0.25,
        sustain_amplitude: 0.75,
        release_time: 0.7,
    };

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    clear_background(BLACK);

    println!("Play notes by pressing keys zxcvbnm,./");

    let mut keys = [
        (KeyCode::Z, NOTE_A),
        (KeyCode::X, NOTE_B),
        (KeyCode::C, NOTE_C),
        (KeyCode::V, NOTE_D),
        (KeyCode::B, NOTE_E),
        (KeyCode::N, NOTE_F),
        (KeyCode::M, NOTE_G),
        (KeyCode::Comma, NOTE_A2),
        (KeyCode::Period, NOTE_B2),
        (KeyCode::Slash, NOTE_C2),
    ].iter().map(|(code, note)| {
        let mut sound = Waveform::new(WaveformType::Sine(envelope.clone()));
        sound.set_frequency(*note);
        stream_handle.play_raw(sound.clone()).unwrap();
        (code, sound)
    }).collect::<Vec<_>>();

    loop {
        keys.iter_mut().for_each(|(key, sound)| {
            if is_key_pressed(**key) {
                sound.note_on()
            } else if is_key_released(**key) {
                sound.note_off()
            }
        });

        next_frame().await
    }
}
