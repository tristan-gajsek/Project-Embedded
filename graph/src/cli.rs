use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
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
    #[arg(
        long,
        short,
        help = "Delay between randomly generating data in milliseconds",
        default_value_t = 1000
    )]
    pub delay: u64,

    #[arg(long, short = 'W', help = "Window width", default_value_t = 1280)]
    pub width: usize,
    #[arg(long, short = 'H', help = "Window height", default_value_t = 720)]
    pub height: usize,
}
