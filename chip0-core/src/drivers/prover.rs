use chip8_core::{error::Chip8Error, util::run_loop};
use p3_machine::{
    config::{default_challenger, default_config, MyConfig},
    machine::Machine,
};
use p3_uni_stark::Val;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use crate::{cpu::ProofRequest, machine::Chip0Machine};

const FREQUENCY: u64 = 60;

// TODO: Generic on SC config
pub struct Prover {
    machine: Chip0Machine,
    config: MyConfig,
}

impl Prover {
    fn new() -> Self {
        let config = default_config();

        Self {
            machine: Chip0Machine::default(),
            config,
        }
    }

    fn frequency(&self) -> u64 {
        FREQUENCY
    }

    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        proving_queue: Arc<RwLock<VecDeque<ProofRequest<Val<MyConfig>>>>>,
    ) {
        run_loop(status.clone(), self.frequency(), move |_| {
            if let Some(request) = proving_queue.write().unwrap().pop_front() {
                let ProofRequest {
                    traces,
                    public_values,
                } = request;

                let (pk, vk) = self.machine.setup(&self.config);
                let mut challenger = default_challenger();

                let traces = traces.into_iter().map(Some).collect::<Vec<_>>();
                self.machine
                    .prove(&self.config, &mut challenger, &pk, traces, public_values);
            }

            Ok(())
        });
    }
}
