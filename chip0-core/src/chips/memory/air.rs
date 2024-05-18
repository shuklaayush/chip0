use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::Matrix;

use super::columns::{MemoryCols, NUM_MEMORY_COLS};
use super::MemoryChip;

impl<F> BaseAir<F> for MemoryChip {
    fn width(&self) -> usize {
        NUM_MEMORY_COLS
    }
}

impl<AB: AirBuilder> Air<AB> for MemoryChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &MemoryCols<AB::Var> = (*local).borrow();
        let next: &MemoryCols<AB::Var> = (*next).borrow();
    }
}
