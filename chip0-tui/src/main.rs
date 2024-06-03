mod args;
mod drivers;
mod terminal;

use args::CmdArgs;
use chip0_core::{config::MyConfig, cpu::StarkCpu, prover::DefaultProver};
use chip8_core::{
    input::{InputEvent, InputKind},
    keypad::Key,
    Chip8,
};
use clap::Parser;
use csv::{Reader, Writer, WriterBuilder};
use drivers::input::CsvRecord;
use eyre::Result;
use rand::{random, rngs::StdRng, SeedableRng};
use std::fs::{self, OpenOptions};
use terminal::{restore_terminal, setup_terminal};
use tracing_forest::{util::LevelFilter, ForestLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

use crate::drivers::{
    audio::TerminalAudio, display::TerminalDisplay, input::TerminalKeyboardInput,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = CmdArgs::parse();

    let rom = fs::read(args.rom)?;

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    Registry::default()
        .with(env_filter)
        .with(ForestLayer::default())
        .init();

    let terminal = setup_terminal(args.headless)?;

    let (inputs, input_writer) = if let Some(input_file) = &args.input_file {
        if !input_file.exists() || args.overwrite {
            let writer = Writer::from_path(input_file)?;
            (vec![], Some(writer))
        } else {
            let mut reader = Reader::from_path(input_file)?;
            let parsed: Vec<(u64, InputEvent)> = reader
                .deserialize()
                .map(|result| {
                    let record: CsvRecord = result?;
                    let key = Key::try_from(record.key)?;
                    let kind = InputKind::try_from(record.kind)?;
                    Ok((record.clk, InputEvent { key, kind }))
                })
                .collect::<Result<_>>()?;
            let f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(input_file)?;
            let writer = WriterBuilder::new()
                .has_headers(parsed.is_empty())
                .from_writer(f);
            (parsed, Some(writer))
        }
    } else {
        (vec![], None)
    };

    let input_driver = TerminalKeyboardInput::new(input_writer);
    let display_driver = {
        if !args.headless {
            Some(TerminalDisplay::new(
                terminal,
                args.refresh_rate,
                args.bg_color,
                args.fg_color,
                args.border_color,
            ))
        } else {
            None
        }
    };
    let audio_driver = {
        if !args.headless {
            Some(TerminalAudio::default())
        } else {
            None
        }
    };

    let seeded_rng = StdRng::seed_from_u64(args.random_seed.unwrap_or(random()));
    let prover = DefaultProver::new(rom.clone());
    let cpu: StarkCpu<_, MyConfig, _> = StarkCpu::new(args.clk_freq, seeded_rng, prover);
    let mut chip8 = Chip8::new(cpu, inputs);
    let res = chip8
        .load_and_run(
            rom.as_slice(),
            args.num_cycles,
            input_driver,
            display_driver,
            audio_driver,
        )
        .await;

    restore_terminal(args.headless)?;
    res?;

    Ok(())
}
