pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, PrimeField32};

#[cfg(feature = "trace-writer")]
use self::columns::MemoryStartCols;

#[derive(Clone, Debug)]
pub struct MemoryStartChip {
    pub rom: Vec<u8>,
    pub bus_memory_start: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for MemoryStartChip {
    fn main_headers(&self) -> Vec<String> {
        MemoryStartCols::<F>::headers()
    }
}
