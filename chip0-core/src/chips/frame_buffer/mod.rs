pub mod air;
pub mod columns;
pub mod interaction;

use p3_air_util::TraceWriter;
use p3_field::{ExtensionField, PrimeField32};

use self::columns::FrameBufferCols;

#[derive(Clone, Debug)]
pub struct FrameBufferChip {
    pub bus_frame_buffer: usize,
    pub bus_range: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for FrameBufferChip {
    fn main_headers(&self) -> Vec<String> {
        FrameBufferCols::<F>::headers()
    }
}
