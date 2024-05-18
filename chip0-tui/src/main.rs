mod args;
mod drivers;
mod terminal;

use args::CmdArgs;
use chip0_core::cpu::StarkCpu;
use chip8_core::{
    input::{InputEvent, InputKind},
    keypad::Key,
    Chip8,
};
use clap::Parser;
use csv::{Reader, Writer, WriterBuilder};
use drivers::{input::CsvRecord, prover::DefaultProverDriver};
use eyre::Result;
use p3_baby_bear::BabyBear;
use p3_machine::config::MyConfig;
use rand::{random, rngs::StdRng, SeedableRng};
use std::fs::{self, OpenOptions};
use terminal::{restore_terminal, setup_terminal};

use crate::drivers::{
    audio::TerminalAudio, display::TerminalDisplay, input::TerminalKeyboardInput,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = CmdArgs::parse();

    let rom = fs::read(args.rom)?;
    let terminal = setup_terminal(args.headless)?;

    let (inputs, input_writer) = if let Some(input_file) = &args.input_file {
        if args.overwrite {
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
    // let prover_driver = Some(DefaultProverDriver::new());
    let prover_driver: Option<DefaultProverDriver> = None;

    let seeded_rng = StdRng::seed_from_u64(args.random_seed.unwrap_or(random()));
    let cpu: StarkCpu<_, MyConfig> = StarkCpu::new(args.clk_freq, seeded_rng);
    let mut chip8 = Chip8::new(cpu, inputs);
    let res = chip8
        .load_and_run(
            rom.as_slice(),
            input_driver,
            display_driver,
            audio_driver,
            prover_driver,
        )
        .await;

    restore_terminal(args.headless)?;
    res?;

    Ok(())
}
