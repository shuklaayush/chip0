pub mod air;
pub mod columns;
pub mod interaction;

use p3_field::{ExtensionField, PrimeField32};
use p3_stark::AirDebug;

use self::columns::HashCols;

#[derive(Clone, Debug)]
pub struct HashChip {}

impl<F: PrimeField32, EF: ExtensionField<F>> AirDebug<F, EF> for HashChip {
    #[cfg(feature = "debug-trace")]
    fn main_headers(&self) -> Vec<String> {
        HashCols::<F>::headers()
    }
}
