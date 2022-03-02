use std::sync::Arc;

use parking_lot::RwLock;
//use rodio::Source;

use super::oscillator::{Oscillator, Waveform};

#[derive(Clone)]
pub struct OscillatorHandle {
    pub oscillator: Arc<RwLock<Oscillator>>,
}

impl OscillatorHandle {
    pub fn new(oscillator: Oscillator) -> Self {
        Self {
            oscillator: Arc::new(RwLock::new(oscillator)),
        }
    }

    pub fn get_active(&self) -> bool {
        self.oscillator.read().active
    }

    pub fn set_frequency(&self, frequency: f32) {
        self.oscillator.write().frequency = frequency
    }

    pub fn set_waveform(&self, waveform: Waveform) {
        self.oscillator.write().waveform = waveform;
    }

    pub fn set_active(&self, active: bool) {
        self.oscillator.write().active = active;
    }

    pub fn write_to_buffer(&mut self, data: &mut [f32]) {
        let mut lock = self.oscillator.write();
        lock.write_to_buffer(data)
    }
}

// impl<const SAMPLE_RATE: u32> Iterator for OscillatorHandle<{ SAMPLE_RATE }> {
//     type Item = f32;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.oscillator.write().next()
//     }
// }

// impl<const SAMPLE_RATE: u32> Source for OscillatorHandle<{ SAMPLE_RATE }> {
//     fn current_frame_len(&self) -> Option<usize> {
//         None
//     }

//     fn channels(&self) -> u16 {
//         1
//     }

//     fn sample_rate(&self) -> u32 {
//         SAMPLE_RATE
//     }

//     fn total_duration(&self) -> Option<std::time::Duration> {
//         None
//     }
// }
