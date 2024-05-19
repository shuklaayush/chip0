use p3_field::PrimeField32;
use p3_machine::config::{default_challenger, default_config};
use p3_machine::config::{Challenger, MyConfig};
use p3_machine::machine::Machine;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::trace::PartialMachineTrace;

use super::machine::Chip0Machine;

#[derive(Clone)]
pub struct DefaultProver<SC>
where
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
{
    machine: Chip0Machine,
    config: SC,
}

impl DefaultProver<MyConfig> {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            machine: Chip0Machine::new(rom),
            config: default_config(),
        }
    }
}

pub trait Prover<SC>
where
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
{
    fn prove(&self, partial_trace: PartialMachineTrace<Val<SC>>);

    fn new_challenger(&self) -> Challenger;
}

impl Prover<MyConfig> for DefaultProver<MyConfig> {
    fn new_challenger(&self) -> Challenger {
        default_challenger()
    }

    fn prove(&self, partial_trace: PartialMachineTrace<Val<MyConfig>>) {
        let (pk, vk) = self.machine.setup(&self.config);

        let traces = partial_trace.get_trace_matrices();
        let public_values = vec![];

        let mut challenger = self.new_challenger();
        let proof = self
            .machine
            .prove(&self.config, &mut challenger, &pk, traces, &public_values);

        // TODO: Avoid clone
        let mut challenger = self.new_challenger();
        self.machine
            .verify(&self.config, &mut challenger, &vk, &proof, &public_values)
            .unwrap();
    }
}
