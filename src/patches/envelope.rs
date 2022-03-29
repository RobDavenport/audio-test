use std::sync::Arc;

use parking_lot::RwLock;

use super::{attenuation_table_u10, attenuation_table_u8, ATTENUATION_MAX};

#[derive(Clone, Debug)]
pub struct EnvelopeDefinition {
    pub(crate) total_level: u8,
    pub(crate) sustain_level: u8,

    pub(crate) attack_rate: u8,
    pub(crate) decay_attack_rate: u8,
    pub(crate) decay_sustain_rate: u8,
    pub(crate) release_rate: u8,
}

impl Default for EnvelopeDefinition {
    fn default() -> Self {
        Self {
            total_level: 0,
            sustain_level: u8::MAX,

            attack_rate: u8::MAX,
            decay_attack_rate: 0,
            decay_sustain_rate: 0,
            release_rate: u8::MAX,
        }
    }
}

impl EnvelopeDefinition {
    pub fn new(
        total_level: u8,
        attack_rate: u8,
        decay_attack_rate: u8,
        sustain_level: u8,
        decay_sustain_rate: u8,
        release_rate: u8,
    ) -> Self {
        Self {
            total_level,
            sustain_level,
            attack_rate,
            decay_attack_rate,
            decay_sustain_rate,
            release_rate,
            ..Default::default()
        }
    }
}

impl EnvelopeDefinition {
    fn get_attack_rate(&self) -> f32 {
        (self.attack_rate as f32 / u8::MAX as f32).powi(3)
    }

    fn get_decay_rate(&self) -> f32 {
        (self.decay_attack_rate as f32 / u8::MAX as f32).powi(3)
    }

    fn get_sustain_rate(&self) -> f32 {
        (self.decay_sustain_rate as f32 / u8::MAX as f32).powi(3)
    }

    fn get_release_rate(&self) -> f32 {
        (self.release_rate as f32 / u8::MAX as f32).powi(3)
    }
}

#[derive(Clone, PartialEq, Debug)]
enum EnvelopePhase {
    Attack,
    Decay,
    Sustain,
    Release,
    Off,
}
#[derive(Clone, Debug)]
pub struct EnvelopeInstance {
    definition: Arc<RwLock<EnvelopeDefinition>>,
    current_attenuation: f32,
    attenuation_rate: f32,
    current_phase: EnvelopePhase,
}

impl EnvelopeInstance {
    pub fn new(definition: Arc<RwLock<EnvelopeDefinition>>) -> Self {
        Self {
            definition,
            current_attenuation: ATTENUATION_MAX as f32,
            attenuation_rate: 0.0,
            current_phase: EnvelopePhase::Off,
        }
    }

    pub fn attenuation(&self) -> f32 {
        attenuation_table_u10(self.current_attenuation as u16)
            * attenuation_table_u8(u8::MAX - self.definition.read().total_level)
    }

    pub fn key_on(&mut self) {
        self.current_phase = EnvelopePhase::Attack;
        self.attenuation_rate = self.definition.read().get_attack_rate();
    }

    pub fn key_off(&mut self) {
        self.current_phase = EnvelopePhase::Release;
        self.attenuation_rate = self.definition.read().get_release_rate();
    }

    fn next_phase(&mut self) {
        match self.current_phase {
            EnvelopePhase::Attack => {
                self.attenuation_rate = self.definition.read().get_decay_rate();
                self.current_phase = EnvelopePhase::Decay;
            }
            EnvelopePhase::Decay => {
                self.attenuation_rate = self.definition.read().get_sustain_rate();
                self.current_phase = EnvelopePhase::Sustain;
            }
            EnvelopePhase::Sustain => {
                self.attenuation_rate = self.definition.read().get_release_rate();
                self.current_phase = EnvelopePhase::Release;
            }
            EnvelopePhase::Release => {
                self.attenuation_rate = 0.0;
                self.current_phase = EnvelopePhase::Off;
            }
            EnvelopePhase::Off => panic!("Called Next phase on Off"),
        };
    }

    pub(crate) fn tick(&mut self) {
        match self.current_phase {
            EnvelopePhase::Attack => {
                self.current_attenuation -= self.attenuation_rate;

                if self.current_attenuation <= 0.0 {
                    self.current_attenuation = 0.0;
                    self.next_phase();
                }
            }
            EnvelopePhase::Decay => {
                self.current_attenuation += self.attenuation_rate;
                let sustain_level = self.definition.read().sustain_level;

                if self.current_attenuation >= (u8::MAX - sustain_level) as f32 {
                    self.current_attenuation = (u8::MAX - sustain_level) as f32;
                    self.next_phase();
                }
            }
            EnvelopePhase::Sustain | EnvelopePhase::Release => {
                self.current_attenuation += self.attenuation_rate;
                if self.current_attenuation >= ATTENUATION_MAX as f32 {
                    self.current_phase = EnvelopePhase::Off;
                    self.attenuation_rate = 0.0;
                    self.current_attenuation = ATTENUATION_MAX as f32;
                }
            }
            EnvelopePhase::Off => (),
        }
    }
}

impl Default for EnvelopeInstance {
    fn default() -> Self {
        Self::new(Arc::new(RwLock::new(EnvelopeDefinition::default())))
    }
}
