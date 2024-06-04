pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

#[cfg(feature = "trace-writer")]
use self::columns::ClearCols;

#[derive(Clone, Debug)]
pub struct ClearChip {
    bus_clear: usize,
    bus_frame_buffer: usize,
}

impl ClearChip {
    pub fn new(bus_clear: usize, bus_frame_buffer: usize) -> Self {
        Self {
            bus_clear,
            bus_frame_buffer,
        }
    }
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for ClearChip {
    fn main_headers(&self) -> Vec<String> {
        ClearCols::<F>::headers()
    }
}
