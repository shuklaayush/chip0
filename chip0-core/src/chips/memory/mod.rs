pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

#[cfg(feature = "trace-writer")]
use self::columns::MemoryCols;

#[derive(Clone, Debug)]
pub struct MemoryChip {
    pub bus_memory_start: usize,
    pub bus_memory: usize,
    pub bus_range: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for MemoryChip {
    fn main_headers(&self) -> Vec<String> {
        MemoryCols::<F>::headers()
    }
}
