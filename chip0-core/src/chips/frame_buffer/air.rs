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

        builder.assert_bool(local.is_read);
        builder.assert_bool(local.is_write);

        builder.assert_zero(local.is_read * local.is_write);
        builder.assert_bool(local.addr_unchanged);

        builder
            .when_transition()
            .when(next.addr_unchanged)
            .assert_eq(local.addr, next.addr);

        let diff = next.diff_limb_lo + next.diff_limb_hi * AB::Expr::from_canonical_u32(1 << 8);
        builder
            .when_transition()
            .when(next.addr_unchanged)
            .assert_eq(diff.clone(), next.clk - local.clk);
        builder
            .when_transition()
            .when(next.is_read + next.is_write)
            .when_ne(next.addr_unchanged, AB::Expr::one())
            .assert_eq(diff, next.addr - local.addr - AB::Expr::one());

        // TODO: Do I need this?
        builder
            .when_transition()
            .when(next.addr_unchanged)
            .when(next.is_read)
            .assert_eq(local.value, next.value);
    }
}
