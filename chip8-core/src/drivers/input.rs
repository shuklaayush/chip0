use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use crate::{
    error::Chip8Error,
    input::{InputEvent, InputQueue},
    rwlock::{CheckedRead, CheckedWrite},
    util::run_loop,
};

pub trait InputDriver: Send {
    fn frequency(&self) -> u64;

    fn poll(&mut self) -> Result<Option<InputEvent>, Chip8Error>;

    fn log_input(&mut self, _clk: u64, _input: InputEvent) -> Result<(), Chip8Error> {
        Ok(())
    }

    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
        clk: Arc<RwLock<u64>>,
    ) {
        run_loop(status.clone(), self.frequency(), move |_| {
            let maybe_event = self.poll()?;

            let clk = *clk.checked_read()?;
            let queue_clk = (*queue.checked_read()?).back_clk();
            if clk >= queue_clk.unwrap_or_default() {
                if let Some(event) = maybe_event {
                    self.log_input(clk, event)?;
                    (*queue.checked_write()?).enqueue(clk, event);
                }
            }
            Ok(())
        });
    }
}
