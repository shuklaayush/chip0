use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::{CpuCols, NUM_CPU_COLS};
use super::CpuChip;

impl<F> BaseAir<F> for CpuChip {
    fn width(&self) -> usize {
        NUM_CPU_COLS
    }
}

impl<AB: AirBuilder> Air<AB> for CpuChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &CpuCols<AB::Var> = (*local).borrow();
        let next: &CpuCols<AB::Var> = (*next).borrow();
    }
}
