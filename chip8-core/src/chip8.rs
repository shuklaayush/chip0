use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use super::{
    cpu::Cpu,
    drivers::{AudioDriver, DisplayDriver, InputDriver},
    error::Chip8Error,
    input::InputEvent,
    rwlock::CheckedRead,
    state::State,
};

pub struct Chip8<C>
where
    C: Cpu,
{
    cpu: C,
    input_queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
}

impl<C> Chip8<C>
where
    C: Cpu,
{
    pub fn new(cpu: C, inputs: Vec<(u64, InputEvent)>) -> Self {
        Self {
            cpu,
            input_queue: Arc::new(RwLock::new(VecDeque::from(inputs))),
        }
    }

    pub fn load(&mut self, bytes: &[u8]) -> Result<(), Chip8Error> {
        self.cpu.state().load_rom(bytes)
    }

    pub async fn run(
        &mut self,
        num_cycles: Option<u64>,
        mut input: impl InputDriver + 'static,
        display: Option<impl DisplayDriver + 'static>,
        audio: Option<impl AudioDriver + 'static>,
    ) -> Result<(), Chip8Error> {
        // Status flag to check if machine is still running
        let status = Arc::new(RwLock::new(Ok(())));

        // Input loop
        let input_handle = {
            let status = status.clone();
            let queue = self.input_queue.clone();
            let clk = self.cpu.state().clk_ptr();

            tokio::spawn(async move { input.run(status, queue, clk) })
        };
        // Render loop
        let display_handle = {
            display.map(|mut display| {
                let status = status.clone();
                let frame_buffer = self.cpu.state().frame_buffer_ptr();
                let clk = self.cpu.state().clk_ptr();

                tokio::spawn(async move { display.run(status, frame_buffer, clk) })
            })
        };
        // Audio loop
        let audio_handle = {
            audio.map(|mut audio| {
                let status = status.clone();
                let sound_timer = self.cpu.state().sound_timer_ptr();

                tokio::spawn(async move { audio.run(status, sound_timer) })
            })
        };

        // CPU loop
        self.cpu
            .run(num_cycles, status.clone(), self.input_queue.clone())
            .await;

        // Wait for all threads
        input_handle
            .await
            .map_err(|e| Chip8Error::AsyncAwaitError(e.to_string()))?;
        if let Some(display_handle) = display_handle {
            display_handle
                .await
                .map_err(|e| Chip8Error::AsyncAwaitError(e.to_string()))?;
        }
        if let Some(audio_handle) = audio_handle {
            audio_handle
                .await
                .map_err(|e| Chip8Error::AsyncAwaitError(e.to_string()))?;
        }

        let res = status.checked_read()?;
        res.clone()
    }

    pub async fn load_and_run(
        &mut self,
        rom: &[u8],
        num_cycles: Option<u64>,
        input: impl InputDriver + 'static,
        display: Option<impl DisplayDriver + 'static>,
        audio: Option<impl AudioDriver + 'static>,
    ) -> Result<(), Chip8Error> {
        self.load(rom)?;
        self.run(num_cycles, input, display, audio).await
    }
}
