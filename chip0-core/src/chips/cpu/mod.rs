pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, PrimeField32};

#[cfg(feature = "trace-writer")]
use self::columns::CpuCols;

#[derive(Clone, Debug)]
pub struct CpuChip {
    pub bus_draw: usize,
    pub bus_memory: usize,
    pub bus_keypad: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for CpuChip {
    fn headers(&self) -> Vec<String> {
        CpuCols::<F>::headers()
    }
}
