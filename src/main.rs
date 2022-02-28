mod waveforms;

use macroquad::prelude::*;
use waveforms::{pulse::Pulse, *};

const DUTY: f64 = 0.5;

#[macroquad::main("audio-test")]
async fn main() {
    let mut sound = Pulse::new();
    sound.set_duty(DUTY);
    sound.set_frequency(NOTE_A);
    sound.play();

    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    clear_background(BLACK);
    stream_handle.play_raw(sound.clone()).unwrap();

    println!("Play notes by pressing keys zxcvbnm,./");

    loop {
        if is_key_down(KeyCode::Z) {
            sound.set_frequency(NOTE_A);
            sound.play();
        } else if is_key_down(KeyCode::X) {
            sound.set_frequency(NOTE_B);
            sound.play();
        } else if is_key_down(KeyCode::C) {
            sound.set_frequency(NOTE_C);
            sound.play();
        } else if is_key_down(KeyCode::V) {
            sound.set_frequency(NOTE_D);
            sound.play();
        } else if is_key_down(KeyCode::B) {
            sound.set_frequency(NOTE_E);
            sound.play();
        } else if is_key_down(KeyCode::N) {
            sound.set_frequency(NOTE_F);
            sound.play();
        } else if is_key_down(KeyCode::M) {
            sound.set_frequency(NOTE_G);
            sound.play();
        } else if is_key_down(KeyCode::Comma) {
            sound.set_frequency(NOTE_A2);
            sound.play();
        } else if is_key_down(KeyCode::Period) {
            sound.set_frequency(NOTE_B2);
            sound.play();
        } else if is_key_down(KeyCode::Slash) {
            sound.set_frequency(NOTE_C2);
            sound.play();
        } else {
            sound.stop();
        }

        next_frame().await
    }
}
