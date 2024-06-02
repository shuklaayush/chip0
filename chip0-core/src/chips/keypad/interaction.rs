use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::KeypadCols, KeypadChip};

impl<F: Field> InteractionAir<F> for KeypadChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = KeypadCols::<F>::col_map();
        vec![
            // Interaction {
            //     fields: vec![
            //         VirtualPairCol::single_main(col_map.clk),
            //         VirtualPairCol::single_main(col_map.index),
            //         VirtualPairCol::single_main(col_map.value),
            //     ],
            //     count: VirtualPairCol::single_main(col_map.is_real),
            //     argument_index: self.bus_keypad,
            // }
        ]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![]
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for KeypadChip {}
