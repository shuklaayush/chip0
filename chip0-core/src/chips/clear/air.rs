use core::borrow::Borrow;
use itertools::Itertools;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::ClearCols;
use super::ClearChip;

impl<F> BaseAir<F> for ClearChip {
    fn width(&self) -> usize {
        ClearCols::<F>::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for ClearChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &ClearCols<AB::Var> = (*local).borrow();
        let next: &ClearCols<AB::Var> = (*next).borrow();

        builder.assert_bool(local.is_real);

        // TODO: Constrain x, y
    }
}
