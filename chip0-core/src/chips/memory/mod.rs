pub mod air;
pub mod columns;
pub mod interaction;

use p3_air_util::TraceWriter;
use p3_field::{ExtensionField, PrimeField32};

use self::columns::MemoryCols;

#[derive(Clone, Debug)]
pub struct MemoryChip {
    pub bus_memory_start: usize,
    pub bus_memory: usize,
    pub bus_range: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for MemoryChip {
    fn main_headers(&self) -> Vec<String> {
        MemoryCols::<F>::headers()
    }
}
