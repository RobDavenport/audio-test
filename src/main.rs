mod notes;
mod patches;
mod sequencer;
mod waveform;

use std::{collections::VecDeque, sync::Arc};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};
use macroquad::prelude::*;
use parking_lot::RwLock;
use sequencer::{SequenceInstance, SequenceInstanceHandle};

use crate::patches::*;
use crate::sequencer::SequenceDefinition;

pub use waveform::Waveform;

const DUTY: f64 = 0.25;
pub const TARGET_SAMPLE_RATE: u32 = 48_000; //48khz

// Other potential target sample rates
//pub const TARGET_SAMPLE_RATE: u32 = 96_000;
//pub const TARGET_SAMPLE_RATE: u32 = 44_100; //44.1 kHz
//pub const TARGET_SAMPLE_RATE: u32 = 22_050; //22.050 khz
//pub const TARGET_SAMPLE_RATE: u32 = 11_025; //11.025 khz
//pub const TARGET_SAMPLE_RATE: u32 = 2048;

pub const TARGET_SAMPLE_TICK_TIME: f32 = 1.0 / TARGET_SAMPLE_RATE as f32;
const GRAPH_WINDOW_MULTIPLIER: f32 = 2.0; // How many samples to store
const GRAPH_X: f32 = 1.0f32 / GRAPH_WINDOW_MULTIPLIER;

fn window_conf() -> Conf {
    Conf {
        window_title: "Audio Test".to_owned(),
        fullscreen: false,
        window_width: 1600,
        window_height: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    notes::generate();
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
        let sound = PatchDefinition::new(sample_rate.0);
        let sound_handle = PatchInstanceHandle::new(PatchInstance::new(Arc::new(sound), 0.0));
        sound_handle.set_frequency(notes::index_to_frequency(index + 35)); //35
        handles.push(sound_handle.clone());
        (code, sound_handle)
    })
    .collect::<Vec<_>>();

    let graph = (0..(screen_width() * GRAPH_WINDOW_MULTIPLIER) as usize)
        .map(|_| 0.0f32)
        .collect::<VecDeque<f32>>();

    let graph = Arc::new(RwLock::new(graph));
    let graph_clone = graph.clone();

    let sequence = SequenceDefinition::test_pattern(sample_rate.0);
    let sequence_instance = SequenceInstance::new(Arc::new(sequence));
    let sequence_handle = SequenceInstanceHandle::new(sequence_instance);

    let _sound_thread = std::thread::spawn(move || {
        let stream = device
            .build_output_stream(
                &config,
                move |data, _| {
                    let graph = graph_clone.clone();
                    let sequence_handle = sequence_handle.clone();
                    // Reset output to zero
                    data.iter_mut().for_each(|data| *data = 0.0);

                    //sequence_callback(data, channels, sequence_handle);
                    data_callback(data, channels, handles.as_mut_slice(), graph);
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

    loop {
        let screen_height = screen_height();
        let mid_screen = screen_height / 2.0;

        keys.iter_mut().for_each(|(key, handle)| {
            if is_key_pressed(**key) {
                handle.set_active(true);
            } else if is_key_released(**key) {
                handle.set_active(false);
            }
        });

        let read = graph.read();
        (0..read.len() as usize - 1).for_each(|index| {
            draw_line(
                index as f32 * GRAPH_X,
                mid_screen - read[index] * 60.0,
                (index as f32 * GRAPH_X) + GRAPH_X,
                mid_screen - read[index + 1] * 60.0,
                1.0,
                GREEN,
            )
        });
        drop(read);

        next_frame().await
    }
}

fn sequence_callback(data: &mut [f32], channels: u16, sequence: SequenceInstanceHandle) {
    sequence.write_to_buffer(data, channels);
}

fn data_callback(
    data: &mut [f32],
    channels: u16,
    handles: &mut [PatchInstanceHandle],
    graph: Arc<RwLock<VecDeque<f32>>>,
) {
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
