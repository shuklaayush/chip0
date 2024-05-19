pub mod air;
pub mod columns;
pub mod interaction;

use p3_field::{ExtensionField, PrimeField32};
use p3_stark::AirDebug;

use self::columns::CpuCols;

#[derive(Clone, Debug)]
pub struct CpuChip {
    pub bus_draw: usize,
}

impl<F: PrimeField32, EF: ExtensionField<F>> AirDebug<F, EF> for CpuChip {
    #[cfg(feature = "debug-trace")]
    fn main_headers(&self) -> Vec<String> {
        CpuCols::<F>::headers()
    }
}
