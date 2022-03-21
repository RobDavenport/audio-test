pub struct Envelope {
    attack_rate: u8,
    decay_attack_rate: u8,
    sustain_level: u8,
    decay_sustain_rate: u8,
    release_rate: u8,

    total_level: u8,

    current_attenuation: u8,
    attenuation_rate: u8,
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            attack_rate: 0,
            decay_attack_rate: 0,
            sustain_level: u8::MAX,
            decay_sustain_rate: 0,
            release_rate: 0,

            total_level: 0,

            attenuation_rate: 0,
            current_attenuation: u8::MAX,
        }
    }
}

enum EnvelopePhase {
    Attack,
    Decay,
    Sustain,
    Release,
}

impl Envelope {
    fn tick(&mut self) -> f32 {
        todo!()
    }

    fn attack_rate(&self) -> f32 {
        todo!()
    }

    fn decay_rate(&self) -> f32 {
        todo!()
    }
}
