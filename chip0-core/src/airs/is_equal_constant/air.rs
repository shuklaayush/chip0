use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::IsEqualConstantCols;

pub struct IsEqualConstantAir(pub u32);

impl<F> BaseAir<F> for IsEqualConstantAir {
    fn width(&self) -> usize {
        IsEqualConstantCols::<F>::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for IsEqualConstantAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let local: &IsEqualConstantCols<AB::Var> = (*local).borrow();

        let diff = local.x - AB::Expr::from_canonical_u32(self.0);
        builder.assert_eq(
            AB::Expr::one() - diff.clone() * local.diff_inv,
            local.is_equal,
        );
    }
}
