pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

#[cfg(feature = "trace-writer")]
use self::columns::KeypadCols;

#[derive(Clone, Debug)]
pub struct KeypadChip {
    bus_keypad: usize,
}

impl KeypadChip {
    pub fn new(bus_keypad: usize) -> Self {
        Self { bus_keypad }
    }
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for KeypadChip {
    fn main_headers(&self) -> Vec<String> {
        KeypadCols::<F>::headers()
    }
}
