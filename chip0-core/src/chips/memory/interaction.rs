use p3_air::VirtualPairCol;
use p3_field::AbstractField;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, InteractionChip};

use crate::chips::memory::columns::MEMORY_COL_MAP;

use super::MemoryChip;

impl<F: AbstractField> InteractionChip<F> for MemoryChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(MEMORY_COL_MAP.addr),
                    VirtualPairCol::single_main(MEMORY_COL_MAP.clk),
                    VirtualPairCol::single_main(MEMORY_COL_MAP.value),
                ],
                count: VirtualPairCol::sum_main(vec![
                    MEMORY_COL_MAP.is_read,
                    MEMORY_COL_MAP.is_write,
                ]),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![VirtualPairCol::single_main(MEMORY_COL_MAP.diff_limb_lo)],
                count: VirtualPairCol::sum_main(vec![
                    MEMORY_COL_MAP.is_read,
                    MEMORY_COL_MAP.is_write,
                ]),
                argument_index: self.bus_range,
            },
            Interaction {
                fields: vec![VirtualPairCol::single_main(MEMORY_COL_MAP.diff_limb_hi)],
                count: VirtualPairCol::sum_main(vec![
                    MEMORY_COL_MAP.is_read,
                    MEMORY_COL_MAP.is_write,
                ]),
                argument_index: self.bus_range,
            },
        ]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_main(MEMORY_COL_MAP.addr),
                VirtualPairCol::single_main(MEMORY_COL_MAP.value),
            ],
            count: VirtualPairCol::single_main(MEMORY_COL_MAP.is_first_read),
            argument_index: self.bus_memory_start,
        }]
    }
}

impl<AB: InteractionAirBuilder> InteractionAir<AB> for MemoryChip {}
