pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

#[cfg(feature = "trace-writer")]
use self::columns::FrameBufferCols;

#[derive(Clone, Debug)]
pub struct FrameBufferChip {
    bus_frame_buffer: usize,
    bus_range: usize,
}

impl FrameBufferChip {
    pub fn new(bus_frame_buffer: usize, bus_range: usize) -> Self {
        Self {
            bus_frame_buffer,
            bus_range,
        }
    }
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for FrameBufferChip {
    fn main_headers(&self) -> Vec<String> {
        FrameBufferCols::<F>::headers()
    }
}
