use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::FrameBufferCols, FrameBufferChip};

impl<F: Field> InteractionAir<F> for FrameBufferChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = FrameBufferCols::<F>::col_map();
        vec![
            // Interaction {
            //     fields: vec![
            //         VirtualPairCol::single_main(col_map.addr),
            //         VirtualPairCol::single_main(col_map.clk),
            //         VirtualPairCol::single_main(col_map.value),
            //     ],
            //     count: VirtualPairCol::sum_main(vec![
            //         col_map.is_read,
            //         col_map.is_write,
            //     ]),
            //     argument_index: self.bus_frame_buffer,
            // },
            Interaction {
                fields: vec![VirtualPairCol::single_main(col_map.diff_limb_lo)],
                count: VirtualPairCol::sum_main(vec![col_map.is_read, col_map.is_write]),
                argument_index: self.bus_range,
            },
            Interaction {
                fields: vec![VirtualPairCol::single_main(col_map.diff_limb_hi)],
                count: VirtualPairCol::sum_main(vec![col_map.is_read, col_map.is_write]),
                argument_index: self.bus_range,
            },
        ]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![]
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for FrameBufferChip {}
