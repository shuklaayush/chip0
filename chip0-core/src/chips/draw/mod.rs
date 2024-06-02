pub mod air;
pub mod columns;
pub mod interaction;

use p3_air_util::TraceWriter;
use p3_field::{ExtensionField, PrimeField32};

use self::columns::DrawCols;

#[derive(Clone, Debug)]
pub struct DrawChip {
    pub bus_draw: usize,
    pub bus_frame_buffer: usize,
    pub bus_memory: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for DrawChip {
    fn main_headers(&self) -> Vec<String> {
        DrawCols::<F>::headers()
    }
}
