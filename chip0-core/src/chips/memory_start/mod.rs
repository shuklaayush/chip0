pub mod air;
pub mod columns;
pub mod interaction;

use p3_field::{ExtensionField, PrimeField32};
use p3_stark::AirDebug;

use self::columns::MemoryStartCols;

#[derive(Clone, Debug)]
pub struct MemoryStartChip {
    pub rom: Vec<u8>,
    pub bus_memory_start: usize,
}

impl<F: PrimeField32, EF: ExtensionField<F>> AirDebug<F, EF> for MemoryStartChip {
    #[cfg(feature = "debug-trace")]
    fn main_headers(&self) -> Vec<String> {
        MemoryStartCols::<F>::headers()
    }
}
