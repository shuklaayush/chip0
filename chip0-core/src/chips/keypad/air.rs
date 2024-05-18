use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::Matrix;

use super::columns::{KeypadCols, NUM_KEYPAD_COLS};
use super::KeypadChip;

impl<F> BaseAir<F> for KeypadChip {
    fn width(&self) -> usize {
        NUM_KEYPAD_COLS
    }
}

impl<AB: AirBuilder> Air<AB> for KeypadChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &KeypadCols<AB::Var> = (*local).borrow();
        let next: &KeypadCols<AB::Var> = (*next).borrow();
    }
}
