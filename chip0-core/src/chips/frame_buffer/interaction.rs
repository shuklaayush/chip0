use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::FrameBufferCols, FrameBufferChip};

impl<F: Field> BaseInteractionAir<F> for FrameBufferChip {
    fn sends_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = FrameBufferCols::from_slice(main_indices);
        vec![
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(col_map.addr),
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::single_main(col_map.value),
                ],
                count: VirtualPairCol::sum_main(vec![col_map.is_read, col_map.is_write]),
                argument_index: self.bus_frame_buffer,
            },
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
}

impl<F: Field> InteractionAir<F> for FrameBufferChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = FrameBufferCols::<F>::col_map();
        self.sends_from_main_indices(col_map.as_slice())
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for FrameBufferChip {}
