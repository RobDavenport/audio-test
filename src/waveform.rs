use std::f32::consts::{FRAC_PI_2, PI, TAU};

//TODO: Build a lookup table instead of Sin each thing?
//TODO: Build a lookup of self.frequency * 2.0 * pi?
//TODO: Calculate a wave's period? to prevent overlooping

#[derive(Debug, Clone)]
pub enum Waveform {
    // Basics
    Sine,
    Square,
    Pulse(f32),
    Saw,
    Triangle,
    Noise,

    // OPL
    HalfSine,
    AbsoluteSine,
    QuarterSine,
    AlternatingSine,
    CamelSine,
    LogarithmicSaw,
    // Other
    //PitchedNoise(u32),
}

impl Default for Waveform {
    fn default() -> Self {
        Self::Sine
    }
}

impl Waveform {
    /// Generates a Sine wave oscilator
    pub fn sine() -> Self {
        Self::Sine
    }

    /// Generates a Pulse wave oscilator.
    pub fn pulse(duty: f32) -> Self {
        assert!(duty < 1.0);
        assert!(duty > 0.0);

        if duty == 0.5 {
            Self::Square
        } else {
            Self::Pulse(duty)
        }
    }

    /// Generates a Square wave oscilator. Equal to pulse(0.5)
    pub fn square() -> Self {
        Self::Square
    }

    /// Generates a Sawtooth wave oscilator
    pub fn saw() -> Self {
        Self::Saw
    }

    /// Generates a Triangle wave oscilator
    pub fn triangle() -> Self {
        Self::Triangle
    }

    /// Generates noise.
    pub fn noise() -> Self {
        Self::Noise
    }

    /// Generates a Half Sine wave oscilator. Produces
    /// a sound if the value is >= 0
    pub fn half_sine() -> Self {
        Self::HalfSine
    }

    /// Generates an Absolute Sine wave oscilator.
    pub fn absolute_sine() -> Self {
        Self::AbsoluteSine
    }

    /// Generates a Quarter Sine wave oscilator. Generates a sound
    /// for the rising part of a sine wave.
    pub fn quarter_sine() -> Self {
        Self::QuarterSine
    }

    /// Generates an Alternating Sine wave oscilator. Produces a wave
    /// with a half-period arch and trough, then silence. Similar to a
    /// sine wave with half of the period.
    pub fn alternating_sine() -> Self {
        Self::AlternatingSine
    }

    /// Generates a Camel Sine wave oscilator. Produces a wave
    /// with two half-period arches, then silence. Similar to
    /// absolute value of an alternating sine oscilator.
    pub fn camel_sine() -> Self {
        Self::CamelSine
    }

    /// Generates a Logarithmic Sawtooth wave oscilator.
    pub fn logarithmic_saw() -> Self {
        Self::LogarithmicSaw
    }

    pub fn func(&self, value: f32) -> f32 {
        match self {
            Self::Sine => value.sin(),
            Self::Pulse(duty) => pulse(value, *duty),
            Self::Square => square(value),
            Self::Saw => ((value % TAU) / PI) - 1.0,
            Self::Triangle => value.sin().asin() / FRAC_PI_2,
            Self::Noise => fastrand::f32(),
            Self::HalfSine => half_sine(value),
            Self::AbsoluteSine => value.sin().abs(),
            Self::QuarterSine => quarter_sine(value),
            Self::AlternatingSine => alternating_sine(value),
            Self::CamelSine => camel_sine(value),
            Self::LogarithmicSaw => logarithmic_saw(value),
            //Self::PitchedNoise(state) => todo!(),
        }
    }
}

fn pulse(value: f32, duty: f32) -> f32 {
    if (value.sin() + 1.0) / 2.0 < duty {
        -1.0
    } else {
        1.0
    }
}

fn square(value: f32) -> f32 {
    1.0_f32.copysign(value.sin())
}

fn half_sine(value: f32) -> f32 {
    let output = value.sin();
    output.is_sign_positive() as u32 as f32 * output
}

fn quarter_sine(value: f32) -> f32 {
    let output = value.sin().abs() * square(value * 2.0);
    if output.is_sign_positive() {
        output
    } else {
        0.0
    }
}

fn alternating_sine(value: f32) -> f32 {
    if value.sin().is_sign_positive() {
        (value * 2.0).sin()
    } else {
        0.0
    }
}

fn camel_sine(value: f32) -> f32 {
    alternating_sine(value).abs()
}

fn logarithmic_saw(value: f32) -> f32 {
    (((value % TAU) - PI) / PI).asin() / -FRAC_PI_2
}
