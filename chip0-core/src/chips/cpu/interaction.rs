use chip8_core::constants::{FLAG_REGISTER, NUM_REGISTERS};
use p3_air::VirtualPairCol;
use p3_field::AbstractField;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, InteractionChip};

use crate::chips::cpu::columns::CPU_COL_MAP;

use super::CpuChip;

impl<F: AbstractField> InteractionChip<F> for CpuChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_main(CPU_COL_MAP.clk),
                VirtualPairCol::single_main(CPU_COL_MAP.index_register),
                VirtualPairCol::single_main(CPU_COL_MAP.vx),
                VirtualPairCol::single_main(CPU_COL_MAP.vy),
            ],
            count: VirtualPairCol::single_main(CPU_COL_MAP.is_draw),
            argument_index: self.bus_draw,
        }]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        let mut interactions = vec![
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(CPU_COL_MAP.clk),
                    VirtualPairCol::single_main(CPU_COL_MAP.registers[FLAG_REGISTER]),
                ],
                count: VirtualPairCol::single_main(CPU_COL_MAP.is_draw),
                argument_index: self.bus_draw,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(CPU_COL_MAP.program_counter),
                    VirtualPairCol::single_main(CPU_COL_MAP.clk),
                    VirtualPairCol::single_main(CPU_COL_MAP.opcode_hi),
                ],
                count: VirtualPairCol::single_main(CPU_COL_MAP.is_real),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::new_main(
                        vec![(CPU_COL_MAP.program_counter, F::one())],
                        F::one(),
                    ),
                    VirtualPairCol::single_main(CPU_COL_MAP.clk),
                    VirtualPairCol::single_main(CPU_COL_MAP.opcode_lo),
                ],
                count: VirtualPairCol::single_main(CPU_COL_MAP.is_real),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(CPU_COL_MAP.index_register),
                    VirtualPairCol::single_main(CPU_COL_MAP.clk),
                    VirtualPairCol::single_main(CPU_COL_MAP.vx_bcd0),
                ],
                count: VirtualPairCol::single_main(CPU_COL_MAP.is_store_bcd),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::new_main(
                        vec![(CPU_COL_MAP.index_register, F::one())],
                        F::one(),
                    ),
                    VirtualPairCol::single_main(CPU_COL_MAP.clk),
                    VirtualPairCol::single_main(CPU_COL_MAP.vx_bcd1),
                ],
                count: VirtualPairCol::single_main(CPU_COL_MAP.is_store_bcd),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::new_main(
                        vec![(CPU_COL_MAP.index_register, F::one())],
                        F::two(),
                    ),
                    VirtualPairCol::single_main(CPU_COL_MAP.clk),
                    VirtualPairCol::single_main(CPU_COL_MAP.vx_bcd2),
                ],
                count: VirtualPairCol::single_main(CPU_COL_MAP.is_store_bcd),
                argument_index: self.bus_memory,
            },
        ];

        interactions.extend((0..NUM_REGISTERS).map(|i| Interaction {
            fields: vec![
                VirtualPairCol::new_main(
                    vec![(CPU_COL_MAP.index_register, F::one())],
                    F::from_canonical_usize(i),
                ),
                VirtualPairCol::single_main(CPU_COL_MAP.clk),
                VirtualPairCol::single_main(CPU_COL_MAP.registers[i]),
            ],
            // TODO: load/store registers
            count: VirtualPairCol::single_main(CPU_COL_MAP.lte_x_sel[i]),
            argument_index: self.bus_memory,
        }));

        interactions
    }
}

impl<AB: InteractionAirBuilder> InteractionAir<AB> for CpuChip {}
