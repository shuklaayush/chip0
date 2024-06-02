pub mod air;
pub mod columns;
pub mod interaction;

use p3_air_util::TraceWriter;
use p3_field::{ExtensionField, PrimeField32};

use self::columns::RangeCols;

#[derive(Clone, Debug)]
pub struct RangeChip {
    pub bus_range: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for RangeChip {
    fn main_headers(&self) -> Vec<String> {
        RangeCols::<F>::headers()
    }
}
