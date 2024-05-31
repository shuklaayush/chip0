use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, InteractionChip};

use crate::chips::range::columns::RANGE_COL_MAP;

use super::RangeChip;

impl<F: Field> InteractionChip<F> for RangeChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![Interaction {
            fields: vec![VirtualPairCol::single_main(RANGE_COL_MAP.value)],
            count: VirtualPairCol::single_main(RANGE_COL_MAP.mult),
            argument_index: self.bus_range,
        }]
    }
}

impl<AB: InteractionAirBuilder> InteractionAir<AB> for RangeChip {}
