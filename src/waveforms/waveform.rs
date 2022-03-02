// use std::sync::Arc;

// use parking_lot::Mutex;

// use fundsp::hacker::*;
// use rodio::Source;

// #[derive(Clone)]
// pub struct Waveform {
//     func: Arc<Mutex<Box<dyn AudioUnit64 + Send>>>,
// }

// const FREQUENCY_TAG: i64 = 0;
// const ACCUMULATOR_TAG: i64 = 1;
// const END_TIME_TAG: i64 = 2;
// const NOTE_STATE_TAG: i64 = 3;
// const VOLUME_TAG: i64 = 4;
// const DUTY_TAG: i64 = 5;

// #[derive(Clone)]
// pub struct Envelope {
//     /// The time from start until reaching start_amplitude
//     pub attack_time: f64,
//     /// The amplitude to reach during the attack
//     pub start_amplitude: f64,
//     /// The time from start_amplitude to sustain_amplitude
//     pub decay_time: f64,
//     /// The amplitude to hold while held
//     pub sustain_amplitude: f64,
//     /// The time from release to faiding to zero
//     pub release_time: f64,
// }
// pub enum WaveformType {
//     Pulse(Envelope, f64),
//     Sine(Envelope),
//     Triangle(Envelope),
//     Saw(Envelope),
// }

// impl WaveformType {
//     fn generate_waveform(self) -> Box<dyn AudioUnit64 + Send> {
//         match self {
//             WaveformType::Pulse(envelope, duty) => Self::pulse_wave(envelope, duty),
//             WaveformType::Sine(envelope) => Self::sine_wave(envelope),
//             WaveformType::Triangle(envelope) => Self::triangle_wave(envelope),
//             WaveformType::Saw(envelope) => Self::saw_wave(envelope),
//         }
//     }

//     fn pulse_wave(envelope: Envelope, duty: f64) -> Box<dyn AudioUnit64 + Send> {
//         Box::new((tag(FREQUENCY_TAG, 0.0) | tag(DUTY_TAG, duty)) >> pulse())
//     }

//     fn sine_wave(envelope: Envelope) -> Box<dyn AudioUnit64 + Send> {
//         let func = tag(FREQUENCY_TAG, 0.0) >> sine();
//         let end_time = tag(NOTE_STATE_TAG, -1.0) * tag(END_TIME_TAG, 0.0);
//         let envelope = end_time
//             >> lfo2(move |t, end_time| {
//                 // Use a positive or zero value to represent no end time
//                 if end_time.is_sign_positive() {
//                     if t < envelope.attack_time {
//                         // Zero to Attack
//                         lerp(0.0, envelope.start_amplitude, t / envelope.attack_time)
//                     } else if t < envelope.decay_time {
//                         // Attack to Sustain
//                         lerp(
//                             envelope.start_amplitude,
//                             envelope.sustain_amplitude,
//                             lerp_between(envelope.attack_time, envelope.decay_time, t),
//                         )
//                     } else {
//                         // Sustain held
//                         envelope.sustain_amplitude
//                     }
//                 } else if end_time < 0.0 {
//                     let end_time = -end_time;
//                     if t < envelope.release_time + end_time {
//                         // Sustain to release
//                         lerp(
//                             envelope.sustain_amplitude,
//                             0.0,
//                             lerp_between(end_time, end_time + envelope.release_time, t),
//                         )
//                     } else {
//                         0.0
//                     }
//                 } else {
//                     0.0
//                 }
//             });
//         let func = envelope * func;
//         let func = func | timer(ACCUMULATOR_TAG);
//         Box::new(func)
//     }

//     fn triangle_wave(envelope: Envelope) -> Box<dyn AudioUnit64 + Send> {
//         Box::new(tag(FREQUENCY_TAG, 0.0) >> triangle())
//     }

//     fn saw_wave(envelope: Envelope) -> Box<dyn AudioUnit64 + Send> {
//         Box::new(tag(FREQUENCY_TAG, 0.0) >> saw())
//     }
// }

// fn lerp_between(start: f64, end: f64, value: f64) -> f64 {
//     (value - start) / (end - start)
// }

// impl Waveform {
//     pub fn new(waveform: WaveformType) -> Self {
//         Self {
//             func: Arc::new(Mutex::new(waveform.generate_waveform())),
//         }
//     }
// }

// impl Waveform {
//     pub fn set_waveform(&mut self, waveform: WaveformType) {
//         *self.func.lock() = waveform.generate_waveform();
//     }

//     pub fn set_frequency(&mut self, frequency: f64) {
//         self.func.lock().set(FREQUENCY_TAG, frequency);
//     }

//     pub fn note_on(&mut self) {
//         let mut lock = self.func.lock();
//         lock.set(END_TIME_TAG, 0.0);
//         lock.set(NOTE_STATE_TAG, 1.0);
//         lock.reset(None);
//     }

//     pub fn note_off(&mut self) {
//         let mut lock = self.func.lock();
//         let accumulator = lock.get(ACCUMULATOR_TAG).unwrap();
//         lock.set(END_TIME_TAG, accumulator);
//         lock.set(NOTE_STATE_TAG, -1.0);
//     }
// }

// impl Iterator for Waveform {
//     type Item = f32;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(self.func.lock().get_mono() as f32)
//     }
// }

// impl Source for Waveform {
//     fn current_frame_len(&self) -> Option<usize> {
//         None
//     }

//     fn channels(&self) -> u16 {
//         1
//     }

//     fn sample_rate(&self) -> u32 {
//         44_100
//     }

//     fn total_duration(&self) -> Option<std::time::Duration> {
//         None
//     }
// }
