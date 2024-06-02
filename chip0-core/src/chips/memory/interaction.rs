use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{MemoryChip, MemoryCols};

impl<F: Field> InteractionAir<F> for MemoryChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = MemoryCols::<F>::col_map();
        vec![
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(col_map.addr),
                    VirtualPairCol::single_main(col_map.clk),
                    VirtualPairCol::single_main(col_map.value),
                ],
                count: VirtualPairCol::sum_main(vec![col_map.is_read, col_map.is_write]),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![VirtualPairCol::single_main(col_map.diff_limb_lo)],
                count: VirtualPairCol::sum_main(vec![col_map.is_read, col_map.is_write]),
                argument_index: self.bus_range,
            },
            Interaction {
                fields: vec![VirtualPairCol::single_main(col_map.diff_limb_hi)],
                count: VirtualPairCol::sum_main(vec![col_map.is_read, col_map.is_write]),
                argument_index: self.bus_range,
            },
        ]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = MemoryCols::<F>::col_map();
        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_main(col_map.addr),
                VirtualPairCol::single_main(col_map.value),
            ],
            count: VirtualPairCol::single_main(col_map.is_first_read),
            argument_index: self.bus_memory_start,
        }]
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for MemoryChip {}
