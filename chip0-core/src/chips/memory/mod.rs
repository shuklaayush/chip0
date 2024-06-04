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
    bus_memory_start: usize,
    bus_memory: usize,
    bus_range: usize,
}

impl MemoryChip {
    pub fn new(bus_memory_start: usize, bus_memory: usize, bus_range: usize) -> Self {
        Self {
            bus_memory_start,
            bus_memory,
            bus_range,
        }
    }
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for MemoryChip {
    fn main_headers(&self) -> Vec<String> {
        MemoryCols::<F>::headers()
    }
}
