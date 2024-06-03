pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, PrimeField32};

#[cfg(feature = "trace-writer")]
use self::columns::HashCols;

#[derive(Clone, Debug)]
pub struct HashChip {}

impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for HashChip {
    #[cfg(feature = "trace-writer")]
    fn headers(&self) -> Vec<String> {
        HashCols::<F>::headers()
    }
}
