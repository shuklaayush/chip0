use p3_air::VirtualPairCol;
use p3_field::AbstractField;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, InteractionChip};

use crate::chips::frame_buffer::columns::FRAME_BUFFER_COL_MAP;

use super::FrameBufferChip;

impl<F: AbstractField> InteractionChip<F> for FrameBufferChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![
            // Interaction {
            //     fields: vec![
            //         VirtualPairCol::single_main(FRAME_BUFFER_COL_MAP.addr),
            //         VirtualPairCol::single_main(FRAME_BUFFER_COL_MAP.clk),
            //         VirtualPairCol::single_main(FRAME_BUFFER_COL_MAP.value),
            //     ],
            //     count: VirtualPairCol::sum_main(vec![
            //         FRAME_BUFFER_COL_MAP.is_read,
            //         FRAME_BUFFER_COL_MAP.is_write,
            //     ]),
            //     argument_index: self.bus_frame_buffer,
            // },
            Interaction {
                fields: vec![VirtualPairCol::single_main(
                    FRAME_BUFFER_COL_MAP.diff_limb_lo,
                )],
                count: VirtualPairCol::sum_main(vec![
                    FRAME_BUFFER_COL_MAP.is_read,
                    FRAME_BUFFER_COL_MAP.is_write,
                ]),
                argument_index: self.bus_range,
            },
            Interaction {
                fields: vec![VirtualPairCol::single_main(
                    FRAME_BUFFER_COL_MAP.diff_limb_hi,
                )],
                count: VirtualPairCol::sum_main(vec![
                    FRAME_BUFFER_COL_MAP.is_read,
                    FRAME_BUFFER_COL_MAP.is_write,
                ]),
                argument_index: self.bus_range,
            },
        ]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![]
    }
}

impl<AB: InteractionAirBuilder> InteractionAir<AB> for FrameBufferChip {}
