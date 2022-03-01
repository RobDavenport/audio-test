use std::sync::{Arc};
use parking_lot::Mutex;

use fundsp::{hacker::*, prelude::PulseWave};
use rodio::Source;

#[derive(Clone)]
pub struct Pulse {
    func: Arc<Mutex<FuncType>>,
}

type FuncType = An<
    Pipe<
        f64,
        Stack<f64, Tagged<f64>, Tagged<f64>>,
        Binop<f64, FrameMul<U1, f64>, Tagged<f64>, PulseWave<f64>>,
    >,
>;

const FREQUENCY_TAG: i64 = 0;
const DUTY_TAG: i64 = 1;
const VOLUME_TAG: i64 = 2;

impl Pulse {
    pub fn new() -> Self {
        let func =
            (tag(FREQUENCY_TAG, 0.0) | tag(DUTY_TAG, 0.0)) >> ((tag(VOLUME_TAG, 0.0)) * pulse());

        Self {
            func: Arc::new(Mutex::new(func)),
        }
    }
}

impl Pulse {
    pub fn set_frequency(&mut self, frequency: f64) {
        self.func.lock().set(FREQUENCY_TAG, frequency);
    }

    pub fn set_duty(&mut self, duty: f64) {
        self.func.lock().set(DUTY_TAG, duty);
    }

    pub fn stop(&mut self) {
        self.func.lock().set(VOLUME_TAG, 0.0);
    }

    pub fn play(&mut self) {
        self.func.lock().set(VOLUME_TAG, 0.15);
    }
}

impl Iterator for Pulse {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.func.lock().get_mono() as f32)
    }
}

impl Source for Pulse {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44_100
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
