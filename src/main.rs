mod notes;
mod patches;
mod waveform;

use std::{collections::VecDeque, sync::Arc};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleRate, StreamConfig,
};
use macroquad::prelude::*;
use parking_lot::RwLock;

use crate::notes::Notes;
use crate::patches::{Algorithm, Patch, PatchHandle};

pub use waveform::Waveform;

const DUTY: f64 = 0.25;
pub const TARGET_SAMPLE_RATE: usize = 48_000; //48khz

// Other potential target sample rates
//pub const TARGET_SAMPLE_RATE: usize = 96_000;
//pub const TARGET_SAMPLE_RATE: usize = 44_100; //44.1 kHz
//pub const TARGET_SAMPLE_RATE: usize = 22_050; //22.050 khz
//pub const TARGET_SAMPLE_RATE: usize = 11_025; //11.025 khz
//pub const TARGET_SAMPLE_RATE: usize = 2048;

pub const TARGET_SAMPLE_TICK_TIME: f32 = 1.0 / TARGET_SAMPLE_RATE as f32;

#[macroquad::main("audio-test")]
async fn main() {
    let notes = Notes::generate();

    patches::init_attenuation_table();

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

    println!("Change waveform by pressing numbers 0 through 9.");
    println!("Play notes by pressing keys from Z to M and ,./ lshift.");

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
        (KeyCode::Minus, Waveform::pulse(0.33)),
        (KeyCode::Equal, Waveform::pulse(0.10)),
        (KeyCode::Backspace, Waveform::Noise),
    ]
    .into_iter()
    .collect::<Vec<_>>();

    let algorithms = [
        (KeyCode::Key1, Algorithm::Zero),
        (KeyCode::Key2, Algorithm::One),
        (KeyCode::Key3, Algorithm::Two),
        (KeyCode::Key4, Algorithm::Three),
        (KeyCode::Key5, Algorithm::Four),
        (KeyCode::Key6, Algorithm::Five),
        (KeyCode::Key7, Algorithm::Six),
        (KeyCode::Key8, Algorithm::Seven),
    ]
    .into_iter()
    .collect::<Vec<_>>();

    let mut handles = Vec::new();

    let mut keys = [
        (KeyCode::LeftShift),
        (KeyCode::Z),
        (KeyCode::S),
        (KeyCode::X),
        (KeyCode::D),
        (KeyCode::C),
        (KeyCode::V),
        (KeyCode::G),
        (KeyCode::B),
        (KeyCode::H),
        (KeyCode::N),
        (KeyCode::J),
        (KeyCode::M),
        (KeyCode::Comma),
        (KeyCode::L),
        (KeyCode::Period),
        (KeyCode::Semicolon),
        (KeyCode::Slash),
        (KeyCode::Apostrophe),
    ]
    .iter()
    .enumerate()
    .map(|(index, code)| {
        let sound = Patch::new(0.0, sample_rate.0);
        let sound_handle = PatchHandle::new(sound);
        sound_handle.set_frequency(notes.index_to_frequency(index + 35));
        handles.push(sound_handle.clone());
        (code, sound_handle)
    })
    .collect::<Vec<_>>();

    let graph = (0..screen_width() as usize * 2)
        .map(|_| 0.0f32)
        .collect::<VecDeque<f32>>();

    let graph = Arc::new(RwLock::new(graph));
    let graph_clone = graph.clone();

    let _sound_thread = std::thread::spawn(move || {
        let stream = device
            .build_output_stream(
                &config,
                move |data, _| {
                    let graph = graph_clone.clone();
                    data_callback(data, channels, handles.as_mut_slice(), graph)
                },
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

    let mut active_waveform = 0;

    loop {
        let screen_height = screen_height();

        keys.iter_mut().for_each(|(key, handle)| {
            if is_key_pressed(**key) {
                handle.set_active(true);
            } else if is_key_released(**key) {
                handle.set_active(false);
            }
        });

        let read = graph.read();
        (0..read.len() - 1).for_each(|index| {
            let mid_screen = screen_height / 2.0;
            draw_line(
                index as f32,
                mid_screen - read[index] * 20.0,
                index as f32 + 0.5,
                mid_screen - read[index + 1] * 20.0,
                1.0,
                GREEN,
            )
        });
        drop(read);

        waveforms
            .iter()
            .enumerate()
            .for_each(|(index, (key, waveform))| {
                if is_key_pressed(*key) {
                    keys.iter_mut()
                        .for_each(|(_, handle)| handle.set_waveform(3, waveform.clone()));
                    println!("Waveform changed: {:?}", waveform);
                    active_waveform = index;
                }
            });

        // algorithms
        //     .iter()
        //     .enumerate()
        //     .for_each(|(index, (key, algorithm))| {
        //         if is_key_pressed(*key) {
        //             keys.iter_mut()
        //                 .for_each(|(_, handle)| handle.set_algorithm(algorithm.clone()));
        //             println!("algorithm changed: {:?}", algorithm);
        //             active_waveform = index;
        //         }
        //     });

        next_frame().await
    }
}

fn data_callback(
    data: &mut [f32],
    channels: u16,
    handles: &mut [PatchHandle],
    graph: Arc<RwLock<VecDeque<f32>>>,
) {
    // Set all channels to silence
    data.iter_mut().for_each(|data| *data = 0.0);

    handles
        .iter_mut()
        .for_each(|handle| handle.write_to_buffer(data, channels));

    // Update the oscilliscope
    let mut graph = graph.write();
    graph.drain(0..data.len() / channels as usize);
    data.iter()
        .step_by(2)
        .for_each(|amplitude| graph.push_back(*amplitude));
}
