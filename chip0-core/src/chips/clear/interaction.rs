use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::ClearCols, ClearChip};

impl<F: Field> BaseInteractionAir<F> for ClearChip {
    fn receives_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = ClearCols::from_usize_slice(main_indices);
        vec![
            Interaction {
                fields: vec![VirtualPairCol::single_main(col_map.clk)],
                count: VirtualPairCol::single_main(col_map.is_start),
                argument_index: self.bus_clear,
            },
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(col_map.addr),
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::constant(F::zero()),
                ],
                count: VirtualPairCol::single_main(col_map.is_real),
                argument_index: self.bus_frame_buffer,
            },
        ]
    }
}

impl<F: Field> InteractionAir<F> for ClearChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = ClearCols::<F>::col_map();
        self.receives_from_main_indices(col_map.as_usize_slice())
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for ClearChip {}
