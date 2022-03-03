mod waveforms;

use std::collections::HashMap;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleRate, StreamConfig,
};
use macroquad::prelude::*;
use waveforms::*;

use crate::waveforms::{
    oscillator::{Oscillator, Waveform},
    oscillator_handle::OscillatorHandle,
    //waveform::{Envelope as Envelope1, WaveformType},
};

const DUTY: f64 = 0.25;

#[macroquad::main("audio-test")]
async fn main() {
    // let envelope = Envelope1 {
    //     attack_time: 0.05,
    //     start_amplitude: 1.0,
    //     decay_time: 0.25,
    //     sustain_amplitude: 0.75,
    //     release_time: 0.7,
    // };

    //let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let sample_rate = supported_config.sample_rate();
    let channels = supported_config.channels();
    println!("got sample rate: {:?}", sample_rate);

    let config: StreamConfig = supported_config.into();

    println!("Play notes by pressing keys zxcvbnm,./");

    let waveforms = [
        (KeyCode::Key1, Waveform::Sine),
        (KeyCode::Key2, Waveform::Square),
        (KeyCode::Key3, Waveform::Saw),
        (KeyCode::Key4, Waveform::Triangle),
        (KeyCode::Key5, Waveform::HalfSine),
        (KeyCode::Key6, Waveform::AbsoluteSine),
        (KeyCode::Key7, Waveform::QuarterSine),
        (KeyCode::Key8, Waveform::AlternatingSine),
        (KeyCode::Key9, Waveform::CamelSine),
        (KeyCode::Key0, Waveform::LogarithmicSaw),
    ]
    .iter()
    .collect::<Vec<_>>();

    let mut handles = Vec::new();

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
    ]
    .iter()
    .map(|(code, note)| {
        let sound = Oscillator::new(Waveform::Noise, sample_rate.0);
        let sound_handle = OscillatorHandle::new(sound);
        sound_handle.set_frequency(*note);
        handles.push(sound_handle.clone());
        (code, sound_handle)
    })
    .collect::<Vec<_>>();

    //let mut graphs = HashMap::new();

    let sound_thread = std::thread::spawn(move || {
        let stream = device
            .build_output_stream(
                &config,
                move |data, _| data_callback(data, channels, handles.as_mut_slice()),
                move |err| {
                    println!("err: {}", err);
                },
            )
            .unwrap();

        stream.play().unwrap();
        loop {
            //TODO: Convert this to await on a channel?
            std::thread::sleep(std::time::Duration::from_secs_f32(6000.0));
        }
    });

    loop {
        keys.iter_mut().for_each(|(key, handle)| {
            if is_key_pressed(**key) {
                handle.set_active(true);
            } else if is_key_released(**key) {
                handle.set_active(false);
            }

            // let oscillator = handle.oscillator.read();

            // if oscillator.active {
            //     graphs
            //         .entry((oscillator.waveform.clone(), key.clone()))
            //         .or_insert_with(|| {
            //             println!("generating new entry");
            //             let mut cloned = oscillator.clone();
            //             cloned.frame = 0;
            //             cloned
            //                 .into_iter()
            //                 .take((screen_width() * 2.0) as usize)
            //                 .enumerate()
            //                 .map(|(x, y)| (x as f32 * 0.5, (screen_height() / 2.0) - (y * 20.0)))
            //                 .collect::<Vec<_>>()
            //         })
            //         .iter()
            //         .for_each(|(x, y)| draw_circle(*x, *y, 0.5, GREEN));
            // }
        });

        waveforms.iter().for_each(|(key, waveform)| {
            if is_key_pressed(*key) {
                keys.iter_mut()
                    .for_each(|(_, handle)| handle.set_waveform(waveform.clone()));
            }
        });

        next_frame().await
    }
}

fn data_callback(data: &mut [f32], channels: u16, handles: &mut [OscillatorHandle]) {
    data.iter_mut().for_each(|data| *data = 0.0);

    handles.iter_mut().for_each(|handle| {
        if handle.get_active() {
            handle.write_to_buffer(data, channels)
        }
    });
}
