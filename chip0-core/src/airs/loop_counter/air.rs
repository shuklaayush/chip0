use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::LoopCounterCols;

pub struct LoopCounterAir;

impl<F> BaseAir<F> for LoopCounterAir {
    fn width(&self) -> usize {
        LoopCounterCols::<F>::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for LoopCounterAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &LoopCounterCols<AB::Var> = (*local).borrow();
        let next: &LoopCounterCols<AB::Var> = (*next).borrow();

        builder.assert_bool(local.is_start);
        builder.when(local.is_start).assert_zero(local.counter);
        builder
            .when_ne(next.is_start, AB::Expr::one())
            .assert_eq(next.counter, local.counter + AB::Expr::one());
    }
}
