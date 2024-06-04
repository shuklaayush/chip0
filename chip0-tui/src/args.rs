use clap::Parser;
use ratatui::style::Color;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
    #[arg(required = true, value_parser)]
    pub rom: PathBuf,

    #[arg(long = "clock-frequency", default_value_t = 560)]
    pub clk_freq: u64,
    #[arg(long, default_value_t = 60)]
    pub refresh_rate: u64,

    #[arg(long)]
    pub num_cycles: Option<u64>,

    #[arg(long, default_value_t = false)]
    pub headless: bool,

    #[arg(long)]
    pub random_seed: Option<u64>,

    #[arg(long = "inputs")]
    pub input_file: Option<PathBuf>,

    #[arg(long, default_value_t = false, requires = "input_file")]
    pub overwrite: bool,

    #[arg(long = "background", default_value_t = Color::Black, conflicts_with="headless")]
    pub bg_color: Color,
    #[arg(long = "foreground", default_value_t = Color::White, conflicts_with="headless")]
    pub fg_color: Color,
    #[arg(long = "border", default_value_t = Color::White, conflicts_with="headless")]
    pub border_color: Color,
}
