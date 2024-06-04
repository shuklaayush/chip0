pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

#[cfg(feature = "trace-writer")]
use self::columns::CpuCols;

#[derive(Clone, Debug)]
pub struct CpuChip {
    bus_clear: usize,
    bus_draw: usize,
    bus_memory: usize,
    bus_keypad: usize,
}

impl CpuChip {
    pub fn new(bus_clear: usize, bus_draw: usize, bus_memory: usize, bus_keypad: usize) -> Self {
        Self {
            bus_clear,
            bus_draw,
            bus_memory,
            bus_keypad,
        }
    }
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for CpuChip {
    fn main_headers(&self) -> Vec<String> {
        CpuCols::<F>::headers()
    }
}
