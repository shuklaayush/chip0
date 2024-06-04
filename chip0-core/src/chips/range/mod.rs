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
    bus_range: usize,
}

impl RangeChip {
    pub fn new(bus_range: usize) -> Self {
        Self { bus_range }
    }
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for RangeChip {
    fn main_headers(&self) -> Vec<String> {
        RangeCols::<F>::headers()
    }
}
