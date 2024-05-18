pub mod air;
pub mod columns;
pub mod interaction;

use p3_field::{ExtensionField, PrimeField32};
use p3_stark::AirDebug;

use self::columns::KeypadCols;

#[derive(Clone, Debug)]
pub struct KeypadChip {}

impl<F: PrimeField32, EF: ExtensionField<F>> AirDebug<F, EF> for KeypadChip {
    #[cfg(feature = "debug-trace")]
    fn main_headers(&self) -> Vec<String> {
        KeypadCols::<F>::headers()
    }
}
