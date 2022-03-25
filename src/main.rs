mod gui;
mod notes;
mod patches;
mod sequencer;
mod waveform;

use std::{collections::VecDeque, sync::Arc};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};
use gui::framework::{Framework, Gui};
// use macroquad::prelude::*;
use parking_lot::RwLock;
use pixels::{Pixels, SurfaceTexture};
use sequencer::{SequenceInstance, SequenceInstanceHandle};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::patches::*;
use crate::sequencer::SequenceDefinition;

pub use waveform::Waveform;

pub const TARGET_SAMPLE_RATE: u32 = 48_000; //48khz

// Other potential target sample rates
//pub const TARGET_SAMPLE_RATE: u32 = 96_000;
//pub const TARGET_SAMPLE_RATE: u32 = 44_100; //44.1 kHz
//pub const TARGET_SAMPLE_RATE: u32 = 22_050; //22.050 khz
//pub const TARGET_SAMPLE_RATE: u32 = 11_025; //11.025 khz
//pub const TARGET_SAMPLE_RATE: u32 = 2048;

pub const TARGET_SAMPLE_TICK_TIME: f32 = 1.0 / TARGET_SAMPLE_RATE as f32;
const GRAPH_WINDOW_MULTIPLIER: f32 = 1.5; // How many samples to store
const GRAPH_X: f32 = 1.0f32 / GRAPH_WINDOW_MULTIPLIER;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 900;

fn main() {
    notes::generate();
    patches::init_attenuation_table();

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

    let sound = Arc::new(RwLock::new(PatchDefinition::new(sample_rate.0)));
    let event_loop = EventLoop::new();
    let window = init_window(&event_loop);

    let graph = (0..(WIDTH as f32 * GRAPH_WINDOW_MULTIPLIER) as usize)
        .map(|_| 0.0f32)
        .collect::<VecDeque<f32>>();

    let graph = Arc::new(RwLock::new(graph));
    let graph_clone = graph.clone();

    let gui = Gui {
        patch_handle: sound.clone(),
        graph_points: graph.clone(),
    };
    let (mut pixels, mut framework) = init_pixels(&window, gui);
    let mut input = WinitInputHelper::new();

    let mut handles = Vec::new();

    let mut keys = [
        (VirtualKeyCode::LShift),
        (VirtualKeyCode::Z),
        (VirtualKeyCode::S),
        (VirtualKeyCode::X),
        (VirtualKeyCode::D),
        (VirtualKeyCode::C),
        (VirtualKeyCode::V),
        (VirtualKeyCode::G),
        (VirtualKeyCode::B),
        (VirtualKeyCode::H),
        (VirtualKeyCode::N),
        (VirtualKeyCode::J),
        (VirtualKeyCode::M),
        (VirtualKeyCode::Comma),
        (VirtualKeyCode::L),
        (VirtualKeyCode::Period),
        (VirtualKeyCode::Semicolon),
        (VirtualKeyCode::Slash),
        (VirtualKeyCode::Apostrophe),
    ]
    .iter()
    .enumerate()
    .map(|(index, code)| {
        let sound_handle = PatchInstanceHandle::new(PatchInstance::new(sound.clone(), 0.0));
        sound_handle.set_frequency(notes::index_to_frequency(index + 35)); //35
        handles.push(sound_handle.clone());
        (code, sound_handle)
    })
    .collect::<Vec<_>>();

    let sequence = SequenceDefinition::test_pattern(sample_rate.0);
    let sequence_instance = SequenceInstance::new(Arc::new(RwLock::new(sequence)));
    let sequence_handle = SequenceInstanceHandle::new(sequence_instance);

    let _sound_thread = std::thread::spawn(move || {
        let stream = device
            .build_output_stream(
                &config,
                move |data, _| {
                    let graph = graph_clone.clone();

                    // Reset output to zero
                    data.iter_mut().for_each(|data| *data = 0.0);

                    //let sequence_handle = sequence_handle.clone();
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

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            keys.iter_mut().for_each(|(key, handle)| {
                if input.key_pressed(**key) {
                    handle.set_active(true);
                } else if input.key_released(**key) {
                    handle.set_active(false);
                }
            });

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                framework.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            // Draw the current frame
            Event::RedrawRequested(_) => {
                // Draw the world

                // Prepare egui
                framework.prepare(&window);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context)?;

                    Ok(())
                });

                // Basic error handling
                if render_result
                    .map_err(|e| println!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
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

fn init_window<T>(event_loop: &EventLoop<T>) -> Window {
    let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    WindowBuilder::new()
        .with_title("Audio Test")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(event_loop)
        .unwrap()
}

fn init_pixels(window: &Window, gui: Gui) -> (Pixels, Framework) {
    let window_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
    let framework = Framework::new(
        window_size.width,
        window_size.height,
        scale_factor,
        &pixels,
        gui,
    );

    (pixels, framework)
}

// OLD CODE FOR DRAWING THE GRAPH:
//     let screen_height = HEIGHT;
//     let mid_screen = screen_height / 2;
//     let read = graph.read();
//     (0..read.len() as usize - 1).for_each(|index| {
//         draw_line(
//             index as f32 * GRAPH_X,
//             mid_screen - read[index] * 60.0,
//             (index as f32 * GRAPH_X) + GRAPH_X,
//             mid_screen - read[index + 1] * 60.0,
//             1.0,
//             GREEN,
//         )
//     });
//     drop(read);
