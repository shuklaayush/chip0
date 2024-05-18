use std::sync::{Arc, RwLock};

use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    error::Chip8Error,
    rwlock::CheckedRead,
    util::run_loop,
};

pub trait DisplayDriver: Send {
    fn frequency(&self) -> u64;

    fn draw(
        &mut self,
        frame_buffer: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        cpu_freq: Option<u64>,
    ) -> Result<(), Chip8Error>;

    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        frame_buffer: Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>>,
        clk: Arc<RwLock<u64>>,
    ) {
        let mut prev_clk = 0;
        run_loop(status.clone(), self.frequency(), move |elapsed| {
            // TODO: Put behind feature flag
            let curr_clk = *clk.checked_read()?;
            let freq = (curr_clk - prev_clk) as f64 / elapsed.as_secs_f64();
            let freq = freq.round() as u64;

            self.draw(*frame_buffer.checked_read()?, Some(freq))?;
            prev_clk = curr_clk;

            Ok(())
        });
    }
}
