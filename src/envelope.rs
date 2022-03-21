use crate::patches::attenuation;

pub struct Envelope {
    attack_rate: u8,
    decay_attack_rate: u8,
    sustain_level: u8,
    decay_sustain_rate: u8,
    release_rate: u8,

    total_level: u8,

    current_attenuation: u8,
    attenuation_rate: u8,
    current_phase: EnvelopePhase,
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
            current_phase: EnvelopePhase::Release,
        }
    }
}

#[derive(Clone, PartialEq)]
enum EnvelopePhase {
    Attack,
    Decay,
    Sustain,
    Release,
}

impl Envelope {
    pub fn key_on(&mut self) {
        self.current_phase = EnvelopePhase::Attack;
        self.attenuation_rate = self.calculate_attack_rate();
    }

    pub fn key_off(&mut self) {
        self.current_phase = EnvelopePhase::Release;
        self.attenuation_rate = self.calculate_release_rate();
    }

    fn next_phase(&mut self) {
        self.current_phase = match self.current_phase {
            EnvelopePhase::Attack => {
                self.attenuation_rate = self.calculate_decay_rate();
                EnvelopePhase::Decay
            }
            EnvelopePhase::Decay => {
                self.attenuation_rate = self.calculate_sustain_rate();
                EnvelopePhase::Sustain
            }
            EnvelopePhase::Sustain => {
                self.attenuation_rate = self.calculate_release_rate();
                EnvelopePhase::Release
            }
            EnvelopePhase::Release => EnvelopePhase::Release,
        }
    }

    pub(crate) fn tick(&mut self) -> f32 {
        if self.current_phase != EnvelopePhase::Attack && self.current_attenuation == u8::MAX {
            0.0
        } else {
            match self.current_phase {
                EnvelopePhase::Attack => {
                    self.current_attenuation = self
                        .current_attenuation
                        .saturating_sub(self.attenuation_rate);

                    if self.current_attenuation == 0 {
                        self.next_phase();
                    }
                }
                EnvelopePhase::Decay => {
                    self.current_attenuation = self
                        .current_attenuation
                        .saturating_add(self.attenuation_rate);

                    if self.current_attenuation <= self.sustain_level {
                        self.next_phase();
                    }
                }
                EnvelopePhase::Sustain | EnvelopePhase::Release => {
                    self.current_attenuation = self
                        .current_attenuation
                        .saturating_add(self.attenuation_rate);
                }
            }

            attenuation(self.current_attenuation)
        }
    }

    pub(crate) fn calculate_attack_rate(&self) -> u8 {
        todo!()
    }

    pub(crate) fn calculate_decay_rate(&self) -> u8 {
        todo!()
    }

    pub(crate) fn calculate_sustain_rate(&self) -> u8 {
        todo!()
    }

    pub(crate) fn calculate_release_rate(&self) -> u8 {
        todo!()
    }
}
