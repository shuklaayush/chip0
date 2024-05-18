use p3_field::{AbstractField, PrimeField32};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{StarkGenericConfig, Val};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use crate::{error::Chip8Error, util::run_loop};

pub struct ProofRequest<F: AbstractField> {
    pub traces: Vec<Option<RowMajorMatrix<F>>>,
    pub public_values: Vec<F>,
}

pub trait ProverDriver<SC>: Send
where
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
{
    fn frequency(&self) -> u64;

    fn new_challenger(&self) -> SC::Challenger;

    fn prove(&self, request: ProofRequest<Val<SC>>);

    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        proving_queue: Arc<RwLock<VecDeque<ProofRequest<Val<SC>>>>>,
    ) where
        Val<SC>: PrimeField32,
    {
        run_loop(status.clone(), self.frequency(), move |_| {
            if let Some(request) = proving_queue.write().unwrap().pop_front() {
                self.prove(request);
            }
            Ok(())
        });
    }
}
