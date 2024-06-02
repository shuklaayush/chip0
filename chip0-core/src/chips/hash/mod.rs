pub mod air;
pub mod columns;
pub mod interaction;

use p3_air_util::TraceWriter;
use p3_field::{ExtensionField, PrimeField32};

use self::columns::HashCols;

#[derive(Clone, Debug)]
pub struct HashChip {}

impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for HashChip {
    #[cfg(feature = "trace-writer")]
    fn main_headers(&self) -> Vec<String> {
        HashCols::<F>::headers()
    }
}
