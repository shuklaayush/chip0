use chip0_core::machine::Chip0Machine;
use chip8_core::drivers::{ProofRequest, ProverDriver};
use p3_machine::config::{default_challenger, default_config};
use p3_machine::config::{Challenger, MyConfig};
use p3_machine::machine::Machine;
use p3_uni_stark::Val;

const FREQUENCY: u64 = 60;

pub struct DefaultProverDriver {
    machine: Chip0Machine,
    config: MyConfig,
}

impl DefaultProverDriver {
    pub fn new() -> Self {
        Self {
            machine: Chip0Machine::default(),
            config: default_config(),
        }
    }
}

impl ProverDriver<MyConfig> for DefaultProverDriver {
    fn frequency(&self) -> u64 {
        FREQUENCY
    }

    fn new_challenger(&self) -> Challenger {
        default_challenger()
    }

    fn prove(&self, request: ProofRequest<Val<MyConfig>>) {
        let ProofRequest {
            traces,
            public_values,
        } = request;

        let (pk, vk) = self.machine.setup(&self.config);

        let traces = traces.into_iter().map(Some).collect::<Vec<_>>();
        let mut challenger = self.new_challenger();
        self.machine
            .prove(&self.config, &mut challenger, &pk, traces, public_values);
    }
}
