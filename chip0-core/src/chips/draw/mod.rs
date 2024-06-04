pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

#[cfg(feature = "trace-writer")]
use self::columns::DrawCols;

#[derive(Clone, Debug)]
pub struct DrawChip {
    bus_draw: usize,
    bus_frame_buffer: usize,
    bus_memory: usize,
}

impl DrawChip {
    pub fn new(bus_draw: usize, bus_frame_buffer: usize, bus_memory: usize) -> Self {
        Self {
            bus_draw,
            bus_frame_buffer,
            bus_memory,
        }
    }
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for DrawChip {
    fn main_headers(&self) -> Vec<String> {
        DrawCols::<F>::headers()
    }
}
