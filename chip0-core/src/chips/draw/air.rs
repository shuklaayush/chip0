use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::Matrix;

use super::columns::{DrawCols, NUM_DRAW_COLS};
use super::DrawChip;

impl<F> BaseAir<F> for DrawChip {
    fn width(&self) -> usize {
        NUM_DRAW_COLS
    }
}

impl<AB: AirBuilder> Air<AB> for DrawChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &DrawCols<AB::Var> = (*local).borrow();
        let next: &DrawCols<AB::Var> = (*next).borrow();
    }
}
