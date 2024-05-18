use chip8_core::{cpu::Cpu, state::Word};
use p3_field::PrimeField32;
use rand::Rng;

use crate::trace::StarkState;

pub struct StarkCpu<R: Rng, F: PrimeField32> {
    state: StarkState<F>,
    clk_freq: u64,
    rng: R,
}

impl<R: Rng, F: PrimeField32> StarkCpu<R, F> {
    pub fn new(clk_freq: u64, rng: R) -> Self {
        Self {
            state: StarkState::default(),
            clk_freq,
            rng,
        }
    }
}

impl<R: Rng, F: PrimeField32> Cpu for StarkCpu<R, F> {
    type State = StarkState<F>;

    fn state(&mut self) -> &mut Self::State {
        &mut self.state
    }

    fn random(&mut self) -> Word {
        self.rng.gen()
    }

    fn frequency(&self) -> u64 {
        self.clk_freq
    }
}
