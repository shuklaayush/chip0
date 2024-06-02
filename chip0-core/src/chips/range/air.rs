use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::RangeCols;
use super::RangeChip;

impl<F> BaseAir<F> for RangeChip {
    fn width(&self) -> usize {
        RangeCols::<F>::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for RangeChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &RangeCols<AB::Var> = (*local).borrow();
        let next: &RangeCols<AB::Var> = (*next).borrow();

        builder
            .when_first_row()
            .assert_eq(local.value, AB::Expr::zero());
        builder
            .when_transition()
            .assert_eq(next.value, local.value + AB::Expr::one());
    }
}
