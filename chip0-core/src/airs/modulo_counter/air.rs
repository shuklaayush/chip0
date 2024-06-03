use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_air_util::builders::SubAirBuilder;
use p3_field::AbstractField;
use p3_matrix::Matrix;

use crate::airs::is_equal_constant::IsEqualConstantAir;

use super::columns::ModuloCounterCols;

pub struct ModuloCounterAir(pub u32);

impl<F> BaseAir<F> for ModuloCounterAir {
    fn width(&self) -> usize {
        ModuloCounterCols::<F>::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for ModuloCounterAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &ModuloCounterCols<AB::Var> = (*local).borrow();
        let next: &ModuloCounterCols<AB::Var> = (*next).borrow();

        let col_map = ModuloCounterCols::<AB::Var>::col_map();

        // Initialize the counter to 0
        builder
            .when_first_row()
            .assert_eq(local.counter, AB::Expr::zero());

        // Check if reached the max value
        let is_equal = IsEqualConstantAir(self.0 - 1);
        let mut sub_builder = SubAirBuilder::new_main(
            builder,
            vec![col_map.counter, col_map.diff_inv, col_map.is_max],
        );
        is_equal.eval(&mut sub_builder);

        // Increment the counter
        builder
            .when_transition()
            .when_ne(local.is_max, AB::Expr::one())
            .assert_eq(next.counter, local.counter + AB::Expr::one());
        // Reset the counter
        builder
            .when_transition()
            .when(local.is_max)
            .assert_zero(next.counter);
    }
}
