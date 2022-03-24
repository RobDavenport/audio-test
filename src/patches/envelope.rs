use std::sync::Arc;

use parking_lot::RwLock;

use super::{attenuation_table_u10, attenuation_table_u8, ATTENUATION_MAX};

const SLOWEST_ATTENUATION_TICK_COUNT: u8 = 11; // Number of shifts
const HIGHEST_ATTENUATION_RATE: u16 = u8::MAX as u16 * 2;

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

#[derive(Clone, PartialEq, Debug)]
enum EnvelopePhase {
    Attack,
    Decay,
    Sustain,
    Release,
}
#[derive(Clone, Debug)]
pub struct EnvelopeInstance {
    definition: Arc<RwLock<EnvelopeDefinition>>,
    current_attenuation: u16,
    attenuation_rate: u16,
    current_phase: EnvelopePhase,
    clock: u16,
    cycles_per_attenuation_tick: u8,
}

impl EnvelopeInstance {
    pub fn new(definition: Arc<RwLock<EnvelopeDefinition>>) -> Self {
        Self {
            definition,
            current_attenuation: ATTENUATION_MAX,
            attenuation_rate: 0,
            current_phase: EnvelopePhase::Release,
            clock: 0,
            cycles_per_attenuation_tick: 0,
        }
    }

    pub fn attenuation(&self) -> f32 {
        attenuation_table_u10(self.current_attenuation)
            * attenuation_table_u8(u8::MAX - self.definition.read().total_level)
    }

    pub fn key_on(&mut self) {
        self.current_phase = EnvelopePhase::Attack;
        self.attenuation_rate = self.calculate_attack_rate();
        self.calculate_cycles_per_tick();
    }

    pub fn key_off(&mut self) {
        self.current_phase = EnvelopePhase::Release;
        self.attenuation_rate = self.calculate_release_rate();
        self.calculate_cycles_per_tick();
    }

    fn next_phase(&mut self) {
        match self.current_phase {
            EnvelopePhase::Attack => {
                self.attenuation_rate = self.calculate_decay_rate();
                self.current_phase = EnvelopePhase::Decay;
                self.calculate_cycles_per_tick();
            }
            EnvelopePhase::Decay => {
                self.attenuation_rate = self.calculate_sustain_rate();
                self.current_phase = EnvelopePhase::Sustain;
                self.calculate_cycles_per_tick();
            }
            EnvelopePhase::Sustain => {
                self.attenuation_rate = self.calculate_release_rate();
                self.current_phase = EnvelopePhase::Release;
                self.calculate_cycles_per_tick();
            }
            EnvelopePhase::Release => (),
        };
    }

    fn calculate_cycles_per_tick(&mut self) {
        let scale = self.attenuation_rate
            / (HIGHEST_ATTENUATION_RATE / SLOWEST_ATTENUATION_TICK_COUNT as u16);
        self.cycles_per_attenuation_tick = SLOWEST_ATTENUATION_TICK_COUNT - scale as u8;
    }

    pub(crate) fn tick(&mut self) {
        self.clock += 1;

        if self.clock < 1 << self.cycles_per_attenuation_tick {
            return;
        }
        self.clock -= 1 << self.cycles_per_attenuation_tick;

        if self.current_phase != EnvelopePhase::Attack
            && self.current_attenuation >= ATTENUATION_MAX
        {
            return;
        } else {
            if self.attenuation_rate == 0 {
                return;
            }

            match self.current_phase {
                EnvelopePhase::Attack => {
                    self.current_attenuation = self.current_attenuation.saturating_sub(1);

                    if self.current_attenuation == 0 {
                        self.next_phase();
                    }
                }
                EnvelopePhase::Decay => {
                    self.current_attenuation += 1;

                    if self.current_attenuation >= self.definition.read().sustain_level as u16 {
                        self.next_phase();
                    }
                }
                EnvelopePhase::Sustain | EnvelopePhase::Release => {
                    self.current_attenuation += 1;
                }
            }
        }
    }

    pub(crate) fn calculate_attack_rate(&self) -> u16 {
        if self.definition.read().attack_rate == 0 {
            return 0;
        } else {
            (self.definition.read().attack_rate as u16) * 2
        }
    }

    pub(crate) fn calculate_decay_rate(&self) -> u16 {
        if self.definition.read().decay_attack_rate == 0 {
            return 0;
        } else {
            (self.definition.read().decay_attack_rate as u16) * 2
        }
    }

    pub(crate) fn calculate_sustain_rate(&self) -> u16 {
        if self.definition.read().decay_sustain_rate == 0 {
            return 0;
        } else {
            (self.definition.read().decay_sustain_rate as u16) * 2
        }
    }

    pub(crate) fn calculate_release_rate(&self) -> u16 {
        if self.definition.read().release_rate == 0 {
            return 0;
        } else {
            (self.definition.read().release_rate as u16) * 2
        }
    }
}

impl Default for EnvelopeInstance {
    fn default() -> Self {
        Self::new(Arc::new(RwLock::new(EnvelopeDefinition::default())))
    }
}
