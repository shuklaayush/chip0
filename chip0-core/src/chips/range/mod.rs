pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

#[cfg(feature = "trace-writer")]
use self::columns::RangeCols;

#[derive(Clone, Debug)]
pub struct RangeChip {
    pub bus_range: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for RangeChip {
    fn headers(&self) -> Vec<String> {
        RangeCols::<F>::headers()
    }
}
