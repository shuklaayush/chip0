use core::borrow::Borrow;
use itertools::Itertools;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::DrawCols;
use super::DrawChip;

impl<F> BaseAir<F> for DrawChip {
    fn width(&self) -> usize {
        DrawCols::<F>::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for DrawChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &DrawCols<AB::Var> = (*local).borrow();
        let next: &DrawCols<AB::Var> = (*next).borrow();

        builder.assert_bool(local.is_real);
        // TODO: Fill diagonally for proper constraints
        builder.assert_bool(local.is_first);
        builder.assert_bool(local.is_last);

        builder
            .when(local.is_first)
            .assert_eq(local.xs, AB::Expr::zero());
        builder
            .when(local.is_first)
            .assert_eq(local.ys, AB::Expr::zero());
        builder
            .when(local.is_first)
            .assert_eq(local.register_flag, local.pixel * local.frame_buffer_y_x);

        // Constraint clk
        builder
            .when_transition()
            .when_ne(local.is_last, AB::Expr::one())
            .assert_eq(next.clk, local.clk);

        // TODO: More constraints
        builder
            .when(local.is_real)
            .when_ne(local.is_last, AB::Expr::one())
            .assert_eq(
                next.ys * AB::Expr::from_wrapped_u64(8) + next.xs,
                local.ys * AB::Expr::from_wrapped_u64(8) + local.xs + AB::Expr::one(),
            );
        builder
            .when(local.is_real)
            .assert_eq(local.flipped, local.pixel * local.frame_buffer_y_x);
        builder
            .when_transition()
            .when_ne(local.is_last, AB::Expr::one())
            .assert_eq(
                next.register_flag,
                local.register_flag + next.pixel * next.frame_buffer_y_x,
            );

        builder.when(local.is_real).assert_eq(
            local.pixel,
            local
                .pixels_bits
                .into_iter()
                .zip_eq(local.sel_7_minus_xs)
                .map(|(b, sel)| b * sel)
                .sum::<AB::Expr>(),
        );
        // TODO: Constrain x, y, is_first_inner
    }
}
