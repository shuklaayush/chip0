use chip8_core::constants::{FONTSET, FONTSET_START_ADDRESS, MEMORY_SIZE, PROGRAM_START_ADDRESS};
use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::Field;
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;

use crate::chips::memory_start::columns::NUM_MEMORY_START_PREPROCESSED_COLS;

use super::columns::{MemoryStartCols, MemoryStartPreprocessedCols, NUM_MEMORY_START_COLS};
use super::MemoryStartChip;

impl<F: Field> BaseAir<F> for MemoryStartChip {
    fn width(&self) -> usize {
        NUM_MEMORY_START_COLS
    }

    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<F>> {
        let num_real_rows = MEMORY_SIZE;
        let num_rows = num_real_rows.next_power_of_two();
        let mut trace = RowMajorMatrix::new(
            vec![F::zero(); num_rows * NUM_MEMORY_START_PREPROCESSED_COLS],
            NUM_MEMORY_START_PREPROCESSED_COLS,
        );
        let (prefix, rows, suffix) = unsafe {
            trace
                .values
                .align_to_mut::<MemoryStartPreprocessedCols<F>>()
        };
        assert!(prefix.is_empty(), "Alignment should match");
        assert!(suffix.is_empty(), "Alignment should match");
        assert_eq!(rows.len(), num_rows);

        for i in 0..MEMORY_SIZE {
            rows[i].addr = F::from_canonical_usize(i);
        }

        let start = FONTSET_START_ADDRESS as usize;
        let end = FONTSET_START_ADDRESS as usize + FONTSET.len();
        for i in start..end {
            rows[i].value = F::from_canonical_u8(FONTSET[i - start]);
        }

        let start = PROGRAM_START_ADDRESS as usize;
        let end = PROGRAM_START_ADDRESS as usize + self.rom.len();
        for i in start..end {
            rows[i].value = F::from_canonical_u8(self.rom[i - start]);
        }

        Some(trace)
    }
}

impl<AB: AirBuilder> Air<AB> for MemoryStartChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &MemoryStartCols<AB::Var> = (*local).borrow();
        let next: &MemoryStartCols<AB::Var> = (*next).borrow();
    }
}
