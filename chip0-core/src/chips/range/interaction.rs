use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::RangeCols, RangeChip};

impl<F: Field> BaseInteractionAir<F> for RangeChip {
    fn receives_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = RangeCols::from_usize_slice(main_indices);
        vec![Interaction {
            fields: vec![VirtualPairCol::single_main(col_map.value)],
            count: VirtualPairCol::single_main(col_map.mult),
            argument_index: self.bus_range,
        }]
    }
}

impl<F: Field> InteractionAir<F> for RangeChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = RangeCols::<F>::col_map();
        self.receives_from_main_indices(col_map.as_usize_slice())
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for RangeChip {}
