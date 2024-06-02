use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::RangeCols, RangeChip};

impl<F: Field> InteractionAir<F> for RangeChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = RangeCols::<F>::col_map();
        vec![Interaction {
            fields: vec![VirtualPairCol::single_main(col_map.value)],
            count: VirtualPairCol::single_main(col_map.mult),
            argument_index: self.bus_range,
        }]
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for RangeChip {}
