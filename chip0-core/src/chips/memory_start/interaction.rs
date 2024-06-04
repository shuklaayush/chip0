use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{
    columns::{MemoryStartCols, MemoryStartPreprocessedCols},
    MemoryStartChip,
};

impl<F: Field> BaseInteractionAir<F> for MemoryStartChip {
    fn sends_from_indices(
        &self,
        preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let preprocessed_col_map = MemoryStartPreprocessedCols::from_slice(preprocessed_indices);
        let col_map = MemoryStartCols::from_slice(main_indices);
        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_preprocessed(preprocessed_col_map.addr),
                VirtualPairCol::single_preprocessed(preprocessed_col_map.value),
            ],
            count: VirtualPairCol::single_main(col_map.mult),
            argument_index: self.bus_memory_start,
        }]
    }
}

impl<F: Field> InteractionAir<F> for MemoryStartChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let preprocessed_col_map = MemoryStartPreprocessedCols::<F>::col_map();
        let main_col_map = MemoryStartCols::<F>::col_map();

        self.sends_from_indices(preprocessed_col_map.as_slice(), main_col_map.as_slice())
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for MemoryStartChip {
    fn preprocessed_width(&self) -> usize {
        MemoryStartPreprocessedCols::<AB::F>::num_cols()
    }
}
