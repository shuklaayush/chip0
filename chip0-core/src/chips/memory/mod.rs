pub mod air;
pub mod columns;
pub mod interaction;

use p3_field::{ExtensionField, PrimeField32};
use p3_stark::AirDebug;

use self::columns::MemoryCols;

#[derive(Clone, Debug)]
pub struct MemoryChip {
    pub bus_range: usize,
    pub bus_memory: usize,
}

impl<F: PrimeField32, EF: ExtensionField<F>> AirDebug<F, EF> for MemoryChip {
    #[cfg(feature = "debug-trace")]
    fn main_headers(&self) -> Vec<String> {
        MemoryCols::<F>::headers()
    }
}
