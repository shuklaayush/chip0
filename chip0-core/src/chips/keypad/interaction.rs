use itertools::Itertools;
use p3_air::VirtualPairCol;
use p3_field::AbstractField;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, InteractionChip};

use super::{columns::KEYPAD_COL_MAP, KeypadChip};

impl<F: AbstractField> InteractionChip<F> for KeypadChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![]
    }
}

impl<AB: InteractionAirBuilder> InteractionAir<AB> for KeypadChip {}
