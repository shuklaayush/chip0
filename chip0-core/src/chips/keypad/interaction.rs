use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, InteractionChip};

use super::{columns::KEYPAD_COL_MAP, KeypadChip};

impl<F: Field> InteractionChip<F> for KeypadChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![
            // Interaction {
            //     fields: vec![
            //         VirtualPairCol::single_main(KEYPAD_COL_MAP.clk),
            //         VirtualPairCol::single_main(KEYPAD_COL_MAP.index),
            //         VirtualPairCol::single_main(KEYPAD_COL_MAP.value),
            //     ],
            //     count: VirtualPairCol::single_main(KEYPAD_COL_MAP.is_real),
            //     argument_index: self.bus_keypad,
            // }
        ]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![]
    }
}

impl<AB: InteractionAirBuilder> InteractionAir<AB> for KeypadChip {}
