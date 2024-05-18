use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::{FrameBufferCols, NUM_FRAME_BUFFER_COLS};
use super::FrameBufferChip;

impl<F> BaseAir<F> for FrameBufferChip {
    fn width(&self) -> usize {
        NUM_FRAME_BUFFER_COLS
    }
}

impl<AB: AirBuilder> Air<AB> for FrameBufferChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &FrameBufferCols<AB::Var> = (*local).borrow();
        let next: &FrameBufferCols<AB::Var> = (*next).borrow();
    }
}
