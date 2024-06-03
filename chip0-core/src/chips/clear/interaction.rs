use chip8_core::constants::DISPLAY_WIDTH;
use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::ClearCols, ClearChip};

impl<F: Field> InteractionAir<F> for ClearChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = ClearCols::<F>::col_map();
        // TODO: Should this be receive?
        vec![Interaction {
            fields: vec![
                VirtualPairCol::new_main(
                    vec![
                        (col_map.y, F::from_canonical_usize(DISPLAY_WIDTH)),
                        (col_map.x, F::one()),
                    ],
                    F::zero(),
                ),
                VirtualPairCol::single_main(col_map.clk),
                VirtualPairCol::constant(F::zero()),
            ],
            count: VirtualPairCol::single_main(col_map.is_real),
            argument_index: self.bus_frame_buffer,
        }]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = ClearCols::<F>::col_map();
        vec![Interaction {
            fields: vec![VirtualPairCol::single_main(col_map.clk)],
            count: VirtualPairCol::single_main(col_map.is_real),
            argument_index: self.bus_clear,
        }]
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for ClearChip {}
