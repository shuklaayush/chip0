use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::CounterCols;

pub struct CounterAir {}

impl<F> BaseAir<F> for CounterAir {
    fn width(&self) -> usize {
        CounterCols::<F>::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for CounterAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &CounterCols<AB::Var> = (*local).borrow();
        let next: &CounterCols<AB::Var> = (*next).borrow();

        builder
            .when_first_row()
            .assert_eq(local.counter, AB::Expr::zero());

        builder
            .when_transition()
            .assert_eq(next.counter, local.counter + AB::Expr::one());
    }
}
