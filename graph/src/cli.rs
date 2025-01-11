use clap::{Parser, ValueEnum};
use derive_more::derive::Display;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[arg(long, short, help = "The data source", default_value_t = Source::default())]
    pub source: Source,

    #[arg(
        long,
        short,
        help = "Path to serial port device file",
        default_value_t = Into::into("/dev/ttyACM0")
    )]
    pub path: Box<str>,
    #[arg(long, short, help = "Baud rate", default_value_t = 115200)]
    pub baud_rate: u32,
    #[arg(long, short, help = "Timeout in seconds")]
    pub timeout: Option<u64>,

    #[arg(long, short = 'W', help = "Window width", default_value_t = 1280)]
    pub width: usize,
    #[arg(long, short = 'H', help = "Window height", default_value_t = 720)]
    pub height: usize,
}

#[derive(Debug, Display, Default, Clone, ValueEnum)]
pub enum Source {
    #[default]
    #[display("serial-port")]
    SerialPort,
    #[display("input")]
    Input,
    #[display("random")]
    Random,
}
