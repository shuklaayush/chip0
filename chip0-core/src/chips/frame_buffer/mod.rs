pub mod air;
pub mod columns;
pub mod interaction;

use p3_field::{ExtensionField, PrimeField32};
use p3_stark::AirDebug;

use self::columns::FrameBufferCols;

#[derive(Clone, Debug)]
pub struct FrameBufferChip {}

impl<F: PrimeField32, EF: ExtensionField<F>> AirDebug<F, EF> for FrameBufferChip {
    #[cfg(feature = "debug-trace")]
    fn main_headers(&self) -> Vec<String> {
        FrameBufferCols::<F>::headers()
    }
}
