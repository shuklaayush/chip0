use chip8_core::constants::{NUM_KEYS, NUM_REGISTERS, OPCODE_SIZE};
use core::borrow::Borrow;
use itertools::Itertools;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::columns::CpuCols;
use super::CpuChip;

impl<F> BaseAir<F> for CpuChip {
    fn width(&self) -> usize {
        CpuCols::<F>::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for CpuChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &CpuCols<AB::Var> = (*local).borrow();
        let next: &CpuCols<AB::Var> = (*next).borrow();

        // is_real is boolean
        builder.assert_bool(local.is_real);

        // clk increments by 1
        builder
            .when_transition()
            .when(next.is_real)
            .assert_eq(next.clk, local.clk + AB::Expr::one());

        // Selectors are boolean
        builder.assert_bool(local.is_clear_display);
        builder.assert_bool(local.is_return);
        builder.assert_bool(local.is_jump);
        builder.assert_bool(local.is_call);
        builder.assert_bool(local.is_skip_equal);
        builder.assert_bool(local.is_skip_not_equal);
        builder.assert_bool(local.is_skip_equal_xy);
        builder.assert_bool(local.is_load);
        builder.assert_bool(local.is_add);
        builder.assert_bool(local.is_move);
        builder.assert_bool(local.is_or);
        builder.assert_bool(local.is_and);
        builder.assert_bool(local.is_xor);
        builder.assert_bool(local.is_add_xy);
        builder.assert_bool(local.is_sub_xy);
        builder.assert_bool(local.is_shift_right);
        builder.assert_bool(local.is_sub_yx);
        builder.assert_bool(local.is_shift_left);
        builder.assert_bool(local.is_skip_not_equal_xy);
        builder.assert_bool(local.is_load_i);
        builder.assert_bool(local.is_jump_v0);
        builder.assert_bool(local.is_random);
        builder.assert_bool(local.is_draw);
        builder.assert_bool(local.is_skip_key_pressed);
        builder.assert_bool(local.is_skip_key_not_pressed);
        builder.assert_bool(local.is_load_delay);
        builder.assert_bool(local.is_wait_key_press);
        builder.assert_bool(local.is_set_delay);
        builder.assert_bool(local.is_set_sound);
        builder.assert_bool(local.is_add_i);
        builder.assert_bool(local.is_load_font);
        builder.assert_bool(local.is_store_bcd);
        builder.assert_bool(local.is_store_registers);
        builder.assert_bool(local.is_load_memory);

        // Only one selector is active
        let selectors_sum = local.is_clear_display
            + local.is_return
            + local.is_jump
            + local.is_call
            + local.is_skip_equal
            + local.is_skip_not_equal
            + local.is_skip_equal_xy
            + local.is_load
            + local.is_add
            + local.is_move
            + local.is_or
            + local.is_and
            + local.is_xor
            + local.is_add_xy
            + local.is_sub_xy
            + local.is_shift_right
            + local.is_sub_yx
            + local.is_shift_left
            + local.is_skip_not_equal_xy
            + local.is_load_i
            + local.is_jump_v0
            + local.is_random
            + local.is_draw
            + local.is_skip_key_pressed
            + local.is_skip_key_not_pressed
            + local.is_load_delay
            + local.is_wait_key_press
            + local.is_set_delay
            + local.is_set_sound
            + local.is_add_i
            + local.is_load_font
            + local.is_store_bcd
            + local.is_store_registers
            + local.is_load_memory;
        builder.when(local.is_real).assert_one(selectors_sum);

        // register selectors
        for i in 0..NUM_REGISTERS {
            builder.assert_bool(local.x_sel[i]);
            builder.assert_bool(local.y_sel[i]);
            // TODO: Add more constraints
            builder.assert_bool(local.lte_x_sel[i]);
        }
        builder
            .when(local.is_real)
            .assert_one(local.x_sel.into_iter().map(|x| x.into()).sum::<AB::Expr>());
        builder
            .when(local.is_real)
            .assert_one(local.y_sel.into_iter().map(|x| x.into()).sum::<AB::Expr>());
        builder.when(local.is_real).assert_eq(
            local.x,
            local
                .x_sel
                .into_iter()
                .enumerate()
                .map(|(i, x)| AB::Expr::from_canonical_usize(i) * x)
                .sum::<AB::Expr>(),
        );
        builder.when(local.is_real).assert_eq(
            local.y,
            local
                .y_sel
                .into_iter()
                .enumerate()
                .map(|(i, y)| AB::Expr::from_canonical_usize(i) * y)
                .sum::<AB::Expr>(),
        );

        builder.assert_eq(
            local.vx,
            local
                .x_sel
                .iter()
                .zip_eq(local.registers.iter())
                .map(|(&sel, &register)| sel * register)
                .sum::<AB::Expr>(),
        );
        builder.assert_eq(
            local.vy,
            local
                .y_sel
                .iter()
                .zip_eq(local.registers.iter())
                .map(|(&sel, &register)| sel * register)
                .sum::<AB::Expr>(),
        );

        // keypad selector
        for i in 0..NUM_KEYS {
            builder.assert_bool(local.vx_sel[i]);
        }
        builder
            .when(local.is_skip_key_pressed + local.is_skip_key_not_pressed)
            .assert_one(local.x_sel.into_iter().map(|x| x.into()).sum::<AB::Expr>());
        builder
            .when(local.is_skip_key_pressed + local.is_skip_key_not_pressed)
            .assert_eq(
                local.vx,
                local
                    .vx_sel
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| AB::Expr::from_canonical_usize(i) * x)
                    .sum::<AB::Expr>(),
            );
        let key_vx = local
            .vx_sel
            .iter()
            .zip_eq(local.keypad.iter())
            .map(|(&sel, &key)| sel * key)
            .sum::<AB::Expr>();

        // is_equal_vx_nn
        let diff_vx_nn = local.vx - local.nn;
        builder.when(local.is_real).assert_eq(
            AB::Expr::one() - diff_vx_nn.clone() * local.diff_vx_nn_inv,
            local.is_equal_vx_nn,
        );
        builder
            .when(local.is_real)
            .assert_zero(local.is_equal_vx_nn * diff_vx_nn);

        // is_equal_vx_vy
        let diff_vx_vy = local.vx - local.vy;
        builder.when(local.is_real).assert_eq(
            AB::Expr::one() - diff_vx_vy.clone() * local.diff_vx_vy_inv,
            local.is_equal_vx_vy,
        );
        builder
            .when(local.is_real)
            .assert_zero(local.is_equal_vx_vy * diff_vx_vy);

        // program counter
        builder
            .when_transition()
            .when(next.is_real)
            .when(
                local.is_clear_display
                    + local.is_load
                    + local.is_move
                    + local.is_or
                    + local.is_and
                    + local.is_xor
                    + local.is_add_xy
                    + local.is_sub_xy
                    + local.is_shift_right
                    + local.is_sub_yx
                    + local.is_shift_left
                    + local.is_load_i
                    + local.is_random
                    + local.is_draw
                    + local.is_load_delay
                    + local.is_wait_key_press
                    + local.is_set_delay
                    + local.is_set_sound
                    + local.is_add_i
                    + local.is_load_font
                    + local.is_store_bcd
                    + local.is_store_registers
                    + local.is_load_memory,
            )
            .assert_eq(
                next.program_counter,
                local.program_counter + AB::Expr::from_canonical_u16(OPCODE_SIZE),
            );

        let stack_top = local
            .stack_pointer_sel
            .iter()
            .zip_eq(local.stack.iter())
            .map(|(&sel, &val)| sel * val)
            .sum::<AB::Expr>();
        builder
            .when_transition()
            .when(next.is_real) // TODO: Check if necessary
            .when(local.is_return)
            .assert_eq(next.program_counter, stack_top);

        builder
            .when_transition()
            .when(next.is_real)
            .when(local.is_jump + local.is_call)
            .assert_eq(next.program_counter, local.nnn);

        builder
            .when_transition()
            .when(next.is_real)
            .when(local.is_jump_v0)
            .assert_eq(next.program_counter, local.registers[0] + local.nnn);

        builder
            .when_transition()
            .when(next.is_real)
            .when(local.is_skip_equal)
            .when(local.is_equal_vx_nn)
            .assert_eq(
                next.program_counter,
                local.program_counter + AB::Expr::from_canonical_u16(OPCODE_SIZE).double(),
            );
        builder
            .when_transition()
            .when(next.is_real)
            .when(local.is_skip_not_equal)
            .when_ne(local.is_equal_vx_nn, AB::Expr::one())
            .assert_eq(
                next.program_counter,
                local.program_counter + AB::Expr::from_canonical_u16(OPCODE_SIZE).double(),
            );

        builder
            .when_transition()
            .when(next.is_real)
            .when(local.is_skip_equal_xy)
            .when(local.is_equal_vx_vy)
            .assert_eq(
                next.program_counter,
                local.program_counter + AB::Expr::from_canonical_u16(OPCODE_SIZE).double(),
            );
        builder
            .when_transition()
            .when(next.is_real)
            .when(local.is_skip_not_equal_xy)
            .when_ne(local.is_equal_vx_vy, AB::Expr::one())
            .assert_eq(
                next.program_counter,
                local.program_counter + AB::Expr::from_canonical_u16(OPCODE_SIZE).double(),
            );
        builder
            .when_transition()
            .when(next.is_real)
            .when(local.is_skip_key_pressed)
            .when(key_vx.clone())
            .assert_eq(
                next.program_counter,
                local.program_counter + AB::Expr::from_canonical_u16(OPCODE_SIZE).double(),
            );
        builder
            .when_transition()
            .when(next.is_real)
            .when(local.is_skip_key_not_pressed)
            .when_ne(key_vx, AB::Expr::one())
            .assert_eq(
                next.program_counter,
                local.program_counter + AB::Expr::from_canonical_u16(OPCODE_SIZE).double(),
            );

        // lte_x_sel only on 2 opcodes
        // builder.when(local.is_store_registers + local.is_load_memory).assert

        // TODO: Constrain bcd_i
    }
}
