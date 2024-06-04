use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::KeypadCols, KeypadChip};

impl<F: Field> BaseInteractionAir<F> for KeypadChip {
    fn sends_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = KeypadCols::from_slice(main_indices);
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
}

impl<F: Field> InteractionAir<F> for KeypadChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = KeypadCols::<F>::col_map();
        self.sends_from_main_indices(col_map.as_slice())
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for KeypadChip {}
