use chip8_core::constants::{FLAG_REGISTER, NUM_KEYS, NUM_REGISTERS};
use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use crate::chips::cpu::columns::CpuCols;

use super::CpuChip;

impl<F: Field> InteractionAir<F> for CpuChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = CpuCols::<F>::col_map();
        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_main(col_map.clk),
                VirtualPairCol::single_main(col_map.index_register),
                VirtualPairCol::single_main(col_map.vx),
                VirtualPairCol::single_main(col_map.vy),
            ],
            count: VirtualPairCol::single_main(col_map.is_draw),
            argument_index: self.bus_draw,
        }]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = CpuCols::<F>::col_map();
        let mut interactions = vec![
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::single_main(col_map.registers[FLAG_REGISTER]),
                ],
                count: VirtualPairCol::single_main(col_map.is_draw),
                argument_index: self.bus_draw,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(col_map.program_counter),
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::single_main(col_map.opcode_hi),
                ],
                count: VirtualPairCol::single_main(col_map.is_real),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::new_main(vec![(col_map.program_counter, F::one())], F::one()),
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::single_main(col_map.opcode_lo),
                ],
                count: VirtualPairCol::single_main(col_map.is_real),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(col_map.index_register),
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::single_main(col_map.vx_bcd0),
                ],
                count: VirtualPairCol::single_main(col_map.is_store_bcd),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::new_main(vec![(col_map.index_register, F::one())], F::one()),
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::single_main(col_map.vx_bcd1),
                ],
                count: VirtualPairCol::single_main(col_map.is_store_bcd),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::new_main(vec![(col_map.index_register, F::one())], F::two()),
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::single_main(col_map.vx_bcd2),
                ],
                count: VirtualPairCol::single_main(col_map.is_store_bcd),
                argument_index: self.bus_memory,
            },
        ];

        interactions.extend((0..NUM_REGISTERS).map(|i| Interaction {
            fields: vec![
                VirtualPairCol::new_main(
                    vec![(col_map.index_register, F::one())],
                    F::from_canonical_usize(i),
                ),
                VirtualPairCol::single_main(col_map.clk),
                VirtualPairCol::single_main(col_map.registers[i]),
            ],
            // TODO: load/store registers
            count: VirtualPairCol::single_main(col_map.lte_x_sel[i]),
            argument_index: self.bus_memory,
        }));

        // TODO: keypad interactions
        // interactions.extend((0..NUM_KEYS).map(|i| Interaction {
        //     fields: vec![
        //         VirtualPairCol::single_main(col_map.clk),
        //         VirtualPairCol::constant(F::from_canonical_usize(i)),
        //         VirtualPairCol::single_main(col_map.keypad[i]),
        //     ],
        //     count: VirtualPairCol::single_main(col_map.keypad[i]),
        //     argument_index: self.bus_memory,
        // }));

        interactions
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for CpuChip {}
