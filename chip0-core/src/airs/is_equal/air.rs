use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::IsEqualCols;

pub struct IsEqualAir {}

impl<F> BaseAir<F> for IsEqualAir {
    fn width(&self) -> usize {
        IsEqualCols::<F>::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for IsEqualAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let local: &IsEqualCols<AB::Var> = (*local).borrow();

        let diff = local.x - local.y;
        builder.assert_eq(
            AB::Expr::one() - diff.clone() * local.diff_inv,
            local.is_equal,
        );
    }
}
