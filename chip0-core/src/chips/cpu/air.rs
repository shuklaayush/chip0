use chip8_core::constants::{NUM_KEYS, NUM_OPCODES, NUM_REGISTERS, OPCODE_SIZE};
use core::borrow::Borrow;
use itertools::Itertools;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_air_util::builders::SubAirBuilder;
use p3_field::AbstractField;
use p3_matrix::Matrix;

use crate::airs::counter::CounterAir;
use crate::airs::selector::SelectorAir;

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

        let col_map = CpuCols::<AB::Var>::col_map();

        // is_real is boolean
        builder.assert_bool(local.is_real);

        // clk
        // TODO: See if can avoid is_real
        let counter = CounterAir {};
        let mut builder_when_next_is_real = builder.when(next.is_real);
        let mut clk_builder =
            SubAirBuilder::new_main(&mut builder_when_next_is_real, vec![col_map.clk]);
        counter.eval(&mut clk_builder);

        // Opcode selectors
        let selector = SelectorAir::<NUM_OPCODES> {};
        let mut builder_when_local_is_real = builder.when(local.is_real);
        let mut selector_builder = SubAirBuilder::new_main(
            &mut builder_when_local_is_real,
            vec![
                col_map.is_clear_display,
                col_map.is_return,
                col_map.is_jump,
                col_map.is_call,
                col_map.is_skip_equal,
                col_map.is_skip_not_equal,
                col_map.is_skip_equal_xy,
                col_map.is_load,
                col_map.is_add,
                col_map.is_move,
                col_map.is_or,
                col_map.is_and,
                col_map.is_xor,
                col_map.is_add_xy,
                col_map.is_sub_xy,
                col_map.is_shift_right,
                col_map.is_sub_yx,
                col_map.is_shift_left,
                col_map.is_skip_not_equal_xy,
                col_map.is_load_i,
                col_map.is_jump_v0,
                col_map.is_random,
                col_map.is_draw,
                col_map.is_skip_key_pressed,
                col_map.is_skip_key_not_pressed,
                col_map.is_load_delay,
                col_map.is_wait_key_press,
                col_map.is_set_delay,
                col_map.is_set_sound,
                col_map.is_add_i,
                col_map.is_load_font,
                col_map.is_store_bcd,
                col_map.is_store_registers,
                col_map.is_load_memory,
            ],
        );
        selector.eval(&mut selector_builder);

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
