use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::ClearCols, ClearChip};

impl<F: Field> InteractionAir<F> for ClearChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = ClearCols::<F>::col_map();
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

impl<AB: InteractionAirBuilder> Rap<AB> for ClearChip {}
