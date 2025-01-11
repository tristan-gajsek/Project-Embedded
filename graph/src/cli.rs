use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(
        long,
        short,
        help = "The device file",
        default_value_t = Into::into("/dev/ttyACM0")
    )]
    device: Box<str>,

    #[arg(long, short, help = "The baud rate", default_value_t = 115200)]
    baud_rate: u32,

    #[arg(long, short, help = "The timeout in seconds")]
    timeout: Option<u32>,

    #[arg(
        long,
        short,
        help = "Simulate received data instead of reading it from a serial port"
    )]
    simulate: bool,
}
