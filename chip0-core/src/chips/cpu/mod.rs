pub mod air;
pub mod columns;
pub mod interaction;

use p3_air_util::TraceWriter;
use p3_field::{ExtensionField, PrimeField32};

use self::columns::CpuCols;

#[derive(Clone, Debug)]
pub struct CpuChip {
    pub bus_draw: usize,
    pub bus_memory: usize,
    pub bus_keypad: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for CpuChip {
    fn main_headers(&self) -> Vec<String> {
        CpuCols::<F>::headers()
    }
}
