use crate::patches::{attenuation_u16, attenuation_u8};

const SLOWEST_ATTENUATION_TICK_COUNT: u8 = 11; // Number of shifts
const HIGHEST_ATTENUATION_RATE: u16 = u8::MAX as u16 * 2;

#[derive(Clone, Debug)]
pub struct Envelope {
    total_level: u8,
    sustain_level: u8,

    attack_rate: u8,
    decay_attack_rate: u8,
    decay_sustain_rate: u8,
    release_rate: u8,

    current_attenuation: u16,
    attenuation_rate: u16,
    current_phase: EnvelopePhase,
    clock: u16,
    cycles_per_attenuation_tick: u8,
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            total_level: 0,
            sustain_level: u8::MAX,

            attack_rate: 0,
            decay_attack_rate: 0,
            decay_sustain_rate: 0,
            release_rate: 0,

            current_attenuation: u16::MAX,
            attenuation_rate: 0,
            current_phase: EnvelopePhase::Release,
            clock: 0,
            cycles_per_attenuation_tick: 0,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum EnvelopePhase {
    Attack,
    Decay,
    Sustain,
    Release,
}

impl Envelope {
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

    pub fn attenuation(&self) -> f32 {
        attenuation_u16(self.current_attenuation) * attenuation_u8(u8::MAX - self.total_level)
    }

    pub fn key_on(&mut self) {
        self.current_phase = EnvelopePhase::Attack;
        self.attenuation_rate = self.calculate_attack_rate();
        self.calculate_cycles_per_tick();
        println!("attenuation rate: {}", self.attenuation_rate);
    }

    pub fn key_off(&mut self) {
        self.current_phase = EnvelopePhase::Release;
        self.attenuation_rate = self.calculate_release_rate();
        self.calculate_cycles_per_tick();
    }

    fn next_phase(&mut self) {
        println!("next phase! curr: {:?}", self.current_phase);
        self.current_phase = match self.current_phase {
            EnvelopePhase::Attack => {
                self.attenuation_rate = self.calculate_decay_rate();
                self.calculate_cycles_per_tick();
                EnvelopePhase::Decay
            }
            EnvelopePhase::Decay => {
                self.attenuation_rate = self.calculate_sustain_rate();
                self.calculate_cycles_per_tick();

                EnvelopePhase::Sustain
            }
            EnvelopePhase::Sustain => {
                self.attenuation_rate = self.calculate_release_rate();
                self.calculate_cycles_per_tick();

                EnvelopePhase::Release
            }
            EnvelopePhase::Release => EnvelopePhase::Release,
        };
        println!("next phase!: next {:?}", self.current_phase);
        println!("next atten rate!: next {:?}", self.attenuation_rate);
    }

    fn calculate_cycles_per_tick(&mut self) {
        let scale = self.attenuation_rate
            / (HIGHEST_ATTENUATION_RATE / SLOWEST_ATTENUATION_TICK_COUNT as u16);
        self.cycles_per_attenuation_tick = SLOWEST_ATTENUATION_TICK_COUNT - scale as u8;
        println!(
            "scale: {}, final: {}",
            scale, self.cycles_per_attenuation_tick
        );
    }

    pub(crate) fn tick(&mut self) {
        self.clock += 1;

        if self.clock < 1 << self.cycles_per_attenuation_tick {
            return;
        }
        self.clock -= 1 << self.cycles_per_attenuation_tick;

        if self.current_phase != EnvelopePhase::Attack && self.current_attenuation == u16::MAX {
            return;
        } else {
            // if self.total_level != 0 {
            //     println!("att: {}, {:?}, rate: {}", self.current_attenuation, self.current_phase, self.attenuation_rate)
            // }

            match self.current_phase {
                EnvelopePhase::Attack => {
                    self.current_attenuation = self
                        .current_attenuation
                        .saturating_sub(self.attenuation_rate as u16);

                    if self.current_attenuation == 0 {
                        self.next_phase();
                    }
                }
                EnvelopePhase::Decay => {
                    self.current_attenuation = self
                        .current_attenuation
                        .saturating_add(self.attenuation_rate as u16);

                    if self.current_attenuation >= self.sustain_level as u16 {
                        self.next_phase();
                    }
                }
                EnvelopePhase::Sustain | EnvelopePhase::Release => {
                    self.current_attenuation = self
                        .current_attenuation
                        .saturating_add(self.attenuation_rate as u16);
                }
            }
        }
    }

    pub(crate) fn calculate_attack_rate(&self) -> u16 {
        if self.attack_rate == 0 {
            return 0;
        } else {
            (self.attack_rate as u16) * 2
        }
    }

    pub(crate) fn calculate_decay_rate(&self) -> u16 {
        if self.decay_attack_rate == 0 {
            return 0;
        } else {
            (self.decay_attack_rate as u16) * 2
        }
    }

    pub(crate) fn calculate_sustain_rate(&self) -> u16 {
        if self.decay_sustain_rate == 0 {
            return 0;
        } else {
            (self.decay_sustain_rate as u16) * 2
        }
    }

    pub(crate) fn calculate_release_rate(&self) -> u16 {
        if self.release_rate == 0 {
            return 0;
        } else {
            (self.release_rate as u16) * 2
        }
    }
}