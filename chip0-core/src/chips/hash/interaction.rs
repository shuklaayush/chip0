use itertools::Itertools;
use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::HASH_COL_MAP, HashChip};

impl<F: Field> BaseInteractionAir<F> for HashChip {}

impl<F: Field> InteractionAir<F> for HashChip {}

impl<AB: InteractionAirBuilder> Rap<AB> for HashChip {}
