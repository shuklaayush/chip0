use chip8_core::constants::FLAG_REGISTER;
use p3_air::VirtualPairCol;
use p3_field::AbstractField;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, InteractionChip};

use crate::chips::cpu::columns::CPU_COL_MAP;

use super::CpuChip;

impl<F: AbstractField> InteractionChip<F> for CpuChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_main(CPU_COL_MAP.clk),
                VirtualPairCol::single_main(CPU_COL_MAP.index_register),
                VirtualPairCol::single_main(CPU_COL_MAP.vx),
                VirtualPairCol::single_main(CPU_COL_MAP.vy),
            ],
            count: VirtualPairCol::single_main(CPU_COL_MAP.is_draw),
            argument_index: self.bus_draw,
        }]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_main(CPU_COL_MAP.clk),
                VirtualPairCol::single_main(CPU_COL_MAP.registers[FLAG_REGISTER]),
            ],
            count: VirtualPairCol::single_main(CPU_COL_MAP.is_draw),
            argument_index: self.bus_draw,
        }]
    }
}

impl<AB: InteractionAirBuilder> InteractionAir<AB> for CpuChip {}
