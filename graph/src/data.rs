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
use rand::random;

#[derive(Debug, PartialEq)]
pub struct NoiseData {
    pub latitude: f64,
    pub longitude: f64,
    pub decibels: f64,
}

impl NoiseData {
    pub fn new(latitude: f64, longitude: f64, decibels: f64) -> Self {
        Self {
            latitude,
            longitude,
            decibels,
        }
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
    let timeout = args
        .timeout
        .map_or(Duration::MAX, |s| Duration::from_secs(s));
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
            Err(e) => eprintln!("{} {e}", "error".red()),
        }
    }
}

pub fn generate_random(sender: Sender<NoiseData>) -> Result<()> {
    loop {
        thread::sleep(Duration::from_secs(1));
        sender.send(NoiseData::new(random(), random(), random()))?;
    }
}
