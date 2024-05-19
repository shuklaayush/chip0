use p3_air::VirtualPairCol;
use p3_field::AbstractField;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, InteractionChip};

use super::{
    columns::{
        MEMORY_START_COL_MAP, MEMORY_START_PREPROCESSED_COL_MAP, NUM_MEMORY_START_PREPROCESSED_COLS,
    },
    MemoryStartChip,
};

impl<F: AbstractField> InteractionChip<F> for MemoryStartChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_preprocessed(MEMORY_START_PREPROCESSED_COL_MAP.addr),
                VirtualPairCol::single_preprocessed(MEMORY_START_PREPROCESSED_COL_MAP.value),
            ],
            count: VirtualPairCol::single_main(MEMORY_START_COL_MAP.mult),
            argument_index: self.bus_memory_start,
        }]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![]
    }
}

impl<AB: InteractionAirBuilder> InteractionAir<AB> for MemoryStartChip {
    fn preprocessed_width(&self) -> usize {
        NUM_MEMORY_START_PREPROCESSED_COLS
    }
}
