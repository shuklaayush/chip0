use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::HashCols;
use super::HashChip;

impl<F> BaseAir<F> for HashChip {
    fn width(&self) -> usize {
        HashCols::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for HashChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &HashCols<AB::Var> = (*local).borrow();
        let next: &HashCols<AB::Var> = (*next).borrow();
    }
}
