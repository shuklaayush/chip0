use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::SelectorCols;

pub struct SelectorAir<const N: usize> {}

impl<F, const N: usize> BaseAir<F> for SelectorAir<N> {
    fn width(&self) -> usize {
        SelectorCols::<F, N>::num_cols()
    }
}

impl<AB: AirBuilder, const N: usize> Air<AB> for SelectorAir<N> {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let local: &SelectorCols<AB::Var, N> = (*local).borrow();

        // Selectors are boolean
        for i in 0..N {
            builder.assert_bool(local.selectors[i]);
        }

        // Only one selector is active
        let sum = local
            .selectors
            .iter()
            .fold(AB::Expr::zero(), |acc, x| acc + *x);
        builder.assert_one(sum);
    }
}
