use std::{
    io::{self, Cursor, Write},
    sync::mpsc::Sender,
    thread,
    time::Duration,
};

use crate::cli::Cli;
use anyhow::{bail, Result};
use byteorder::{BigEndian, ReadBytesExt};
use colored::Colorize;
use plotters::{
    prelude::Circle,
    style::{Color, RGBAColor, ShapeStyle},
};
use rand::{thread_rng, Rng};
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct NoiseData {
    pub latitude: f64,
    pub longitude: f64,
    pub decibels: f64,
}

impl NoiseData {
    pub const LATITUDE_RANGE: Range<f64> = -90.0..90.0;
    pub const LONGITUDE_RANGE: Range<f64> = -180.0..180.0;
    pub const DECIBEL_RANGE: Range<f64> = 50.0..150.0;

    pub fn new(latitude: f64, longitude: f64, decibels: f64) -> Self {
        Self {
            latitude,
            longitude,
            decibels,
        }
    }

    pub fn size(&self) -> u32 {
        (self.decibels / 5.0) as u32
    }

    pub fn style(&self) -> ShapeStyle {
        let (min, max) = (Self::DECIBEL_RANGE.start, Self::DECIBEL_RANGE.end);
        let r = (self.decibels.clamp(min, max) - min) / (max - min);
        let g = 1.0 - r;
        RGBAColor(
            (r * u8::MAX as f64) as u8,
            (g * u8::MAX as f64) as u8,
            0,
            0.5,
        )
        .filled()
    }

    pub fn circle(&self) -> Circle<(f64, f64), u32> {
        Circle::new((self.latitude, self.longitude), self.size(), self.style())
    }

    fn parse_data(data: &[u8]) -> Result<Self> {
        let mut data = Cursor::new(data);
        if data.read_u16::<BigEndian>()? != 0xABCD {
            bail!("Invalid header");
        }
        Ok(NoiseData {
            latitude: data.read_f64::<BigEndian>()?,
            longitude: data.read_f64::<BigEndian>()?,
            decibels: data.read_f64::<BigEndian>()?,
        })
    }

    fn parse_input(input: &str) -> Result<Self> {
        let floats = input
            .split_whitespace()
            .map(|s| s.parse().map_err(anyhow::Error::from))
            .collect::<Result<Vec<_>>>()?;
        if floats.len() != 3 {
            bail!("Couldn't parse noise data");
        }
        Ok(NoiseData {
            latitude: floats[0],
            longitude: floats[1],
            decibels: floats[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::data::NoiseData;

    #[test]
    fn input_parsing() {
        assert_eq!(
            NoiseData::new(1.0, 2.0, 3.0),
            NoiseData::parse_input("1 2 3").expect("Couldn't parse data"),
        );
        matches!(NoiseData::parse_input(" 1 2 3 "), Err(_));
        matches!(NoiseData::parse_input("1 2 3 4"), Err(_));
    }
}

pub fn read_serial_port(args: &Cli, sender: Sender<NoiseData>) -> Result<()> {
    let timeout = args.timeout.map_or(Duration::MAX, Duration::from_secs);
    let mut port = serialport::new(args.path.as_ref(), args.baud_rate)
        .timeout(timeout)
        .open()?;

    loop {
        let mut buffer = [0; 2 + 3 * size_of::<f64>()];
        port.read_exact(&mut buffer)?;
        sender.send(NoiseData::parse_data(&buffer)?)?;
    }
}

pub fn read_input(sender: Sender<NoiseData>) -> Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = "".to_string();
        io::stdin().read_line(&mut input)?;
        match NoiseData::parse_input(input.trim()) {
            Ok(noise) => sender.send(noise)?,
            Err(e) => eprintln!("{} {e}", "error:".red()),
        }
    }
}

pub fn generate_random(args: &Cli, sender: Sender<NoiseData>) -> Result<()> {
    loop {
        thread::sleep(Duration::from_millis(args.delay));
        sender.send(NoiseData::new(
            thread_rng().gen_range(NoiseData::LATITUDE_RANGE),
            thread_rng().gen_range(NoiseData::LONGITUDE_RANGE),
            thread_rng().gen_range(NoiseData::DECIBEL_RANGE),
        ))?;
    }
}
