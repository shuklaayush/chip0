use itertools::Itertools;
use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, InteractionChip};

use super::{columns::HASH_COL_MAP, HashChip};

impl<F: Field> InteractionChip<F> for HashChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        vec![]
    }

    fn receives(&self) -> Vec<Interaction<F>> {
        vec![]
    }
}

impl<AB: InteractionAirBuilder> InteractionAir<AB> for HashChip {}
