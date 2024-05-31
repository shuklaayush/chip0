use chip8_core::constants::DISPLAY_WIDTH;
use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, InteractionChip};

use crate::chips::draw::columns::DRAW_COL_MAP;

use super::DrawChip;

impl<F: Field> InteractionChip<F> for DrawChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_main(DRAW_COL_MAP.clk),
                VirtualPairCol::single_main(DRAW_COL_MAP.register_flag),
            ],
            count: VirtualPairCol::single_main(DRAW_COL_MAP.is_last),
            argument_index: self.bus_draw,
        }]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(DRAW_COL_MAP.clk),
                    VirtualPairCol::single_main(DRAW_COL_MAP.index_register),
                    VirtualPairCol::single_main(DRAW_COL_MAP.register_x),
                    VirtualPairCol::single_main(DRAW_COL_MAP.register_y),
                ],
                count: VirtualPairCol::single_main(DRAW_COL_MAP.is_first),
                argument_index: self.bus_draw,
            },
            // TODO
            // Interaction {
            //     fields: vec![
            //         VirtualPairCol::new_main(
            //             vec![
            //                 (DRAW_COL_MAP.y, F::from_canonical_usize(DISPLAY_WIDTH)),
            //                 (DRAW_COL_MAP.x, F::one()),
            //             ],
            //             F::zero(),
            //         ),
            //         VirtualPairCol::single_main(DRAW_COL_MAP.clk),
            //         VirtualPairCol::single_main(DRAW_COL_MAP.frame_buffer_y_x),
            //     ],
            //     count: VirtualPairCol::single_main(DRAW_COL_MAP.is_real),
            //     argument_index: self.bus_frame_buffer,
            // },
            // Interaction {
            //     fields: vec![
            //         VirtualPairCol::new_main(
            //             vec![
            //                 (DRAW_COL_MAP.y, F::from_canonical_usize(DISPLAY_WIDTH)),
            //                 (DRAW_COL_MAP.x, F::one()),
            //             ],
            //             F::zero(),
            //         ),
            //         VirtualPairCol::single_main(DRAW_COL_MAP.clk),
            //         VirtualPairCol::new_main(
            //             vec![(DRAW_COL_MAP.frame_buffer_y_x, -F::one())],
            //             F::one(),
            //         ),
            //     ],
            //     count: VirtualPairCol::single_main(DRAW_COL_MAP.pixel),
            //     argument_index: self.bus_frame_buffer,
            // },
            Interaction {
                fields: vec![
                    VirtualPairCol::sum_main(vec![DRAW_COL_MAP.index_register, DRAW_COL_MAP.ys]),
                    VirtualPairCol::single_main(DRAW_COL_MAP.clk),
                    VirtualPairCol::single_main(DRAW_COL_MAP.pixels),
                ],
                count: VirtualPairCol::single_main(DRAW_COL_MAP.is_first_inner),
                argument_index: self.bus_memory,
            },
        ]
    }
}

impl<AB: InteractionAirBuilder> InteractionAir<AB> for DrawChip {}
