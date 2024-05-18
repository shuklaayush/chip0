mod air;
mod columns;
mod interaction;

use p3_field::{ExtensionField, PrimeField32};
use p3_stark::AirDebug;

use self::columns::MemoryCols;

#[derive(Clone, Debug)]
pub struct MemoryChip {}

impl<F: PrimeField32, EF: ExtensionField<F>> AirDebug<F, EF> for MemoryChip {
    #[cfg(feature = "debug-trace")]
    fn main_headers(&self) -> Vec<String> {
        MemoryCols::<F>::headers()
    }
}
