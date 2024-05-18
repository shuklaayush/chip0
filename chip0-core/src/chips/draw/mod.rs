pub mod air;
pub mod columns;
pub mod interaction;

use p3_field::{ExtensionField, PrimeField32};
use p3_stark::AirDebug;

use self::columns::DrawCols;

#[derive(Clone, Debug)]
pub struct DrawChip {}

impl<F: PrimeField32, EF: ExtensionField<F>> AirDebug<F, EF> for DrawChip {
    #[cfg(feature = "debug-trace")]
    fn main_headers(&self) -> Vec<String> {
        DrawCols::<F>::headers()
    }
}
