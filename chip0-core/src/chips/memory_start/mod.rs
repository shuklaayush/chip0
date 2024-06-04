pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

#[cfg(feature = "trace-writer")]
use self::columns::{MemoryStartCols, MemoryStartPreprocessedCols};

#[derive(Clone, Debug)]
pub struct MemoryStartChip {
    rom: Vec<u8>,
    bus_memory_start: usize,
}

impl MemoryStartChip {
    pub fn new(rom: Vec<u8>, bus_memory_start: usize) -> Self {
        Self {
            rom,
            bus_memory_start,
        }
    }
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for MemoryStartChip {
    fn preprocessed_headers(&self) -> Vec<String> {
        MemoryStartPreprocessedCols::<F>::headers()
    }

    fn main_headers(&self) -> Vec<String> {
        MemoryStartCols::<F>::headers()
    }
}
