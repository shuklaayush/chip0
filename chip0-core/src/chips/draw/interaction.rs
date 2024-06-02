use chip8_core::constants::DISPLAY_WIDTH;
use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::DrawCols, DrawChip};

impl<F: Field> InteractionAir<F> for DrawChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = DrawCols::<F>::col_map();
        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_main(col_map.clk),
                VirtualPairCol::single_main(col_map.register_flag),
            ],
            count: VirtualPairCol::single_main(col_map.is_last),
            argument_index: self.bus_draw,
        }]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = DrawCols::<F>::col_map();
        vec![
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::single_main(col_map.index_register),
                    VirtualPairCol::single_main(col_map.register_x),
                    VirtualPairCol::single_main(col_map.register_y),
                ],
                count: VirtualPairCol::single_main(col_map.is_first),
                argument_index: self.bus_draw,
            },
            // TODO
            // Interaction {
            //     fields: vec![
            //         VirtualPairCol::new_main(
            //             vec![
            //                 (col_map.y, F::from_canonical_usize(DISPLAY_WIDTH)),
            //                 (col_map.x, F::one()),
            //             ],
            //             F::zero(),
            //         ),
            //         VirtualPairCol::single_main(col_map.clk),
            //         VirtualPairCol::single_main(col_map.frame_buffer_y_x),
            //     ],
            //     count: VirtualPairCol::single_main(col_map.is_real),
            //     argument_index: self.bus_frame_buffer,
            // },
            // Interaction {
            //     fields: vec![
            //         VirtualPairCol::new_main(
            //             vec![
            //                 (col_map.y, F::from_canonical_usize(DISPLAY_WIDTH)),
            //                 (col_map.x, F::one()),
            //             ],
            //             F::zero(),
            //         ),
            //         VirtualPairCol::single_main(col_map.clk),
            //         VirtualPairCol::new_main(
            //             vec![(col_map.frame_buffer_y_x, -F::one())],
            //             F::one(),
            //         ),
            //     ],
            //     count: VirtualPairCol::single_main(col_map.pixel),
            //     argument_index: self.bus_frame_buffer,
            // },
            Interaction {
                fields: vec![
                    VirtualPairCol::sum_main(vec![col_map.index_register, col_map.ys]),
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::single_main(col_map.pixels),
                ],
                count: VirtualPairCol::single_main(col_map.is_first_inner),
                argument_index: self.bus_memory,
            },
        ]
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for DrawChip {}
