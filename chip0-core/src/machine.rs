use p3_field::PrimeField32;
use p3_machine::machine::Machine;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::chips::Chip0MachineChip;

pub struct Chip0Machine {}

pub enum Chip0MachineBus {}

impl<'a, SC> Machine<'a, SC, Chip0MachineChip> for Chip0Machine
where
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
{
    fn chips(&self) -> Vec<Chip0MachineChip> {
        vec![]
    }
}
