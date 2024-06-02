pub mod air;
pub mod columns;
pub mod interaction;

use p3_air_util::TraceWriter;
use p3_field::{ExtensionField, PrimeField32};

use self::columns::KeypadCols;

#[derive(Clone, Debug)]
pub struct KeypadChip {
    pub bus_keypad: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for KeypadChip {
    fn main_headers(&self) -> Vec<String> {
        KeypadCols::<F>::headers()
    }
}
